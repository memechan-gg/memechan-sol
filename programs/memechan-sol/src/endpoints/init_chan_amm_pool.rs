use crate::consts::{CHAN_MINT, FEE_KEY};
use crate::libraries::MulDiv;
use crate::models::chan_swap::ChanSwap;
use crate::models::staking::StakingPool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::Metadata;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use dynamic_amm::program::DynamicAmm as MeteoraAmm;
use dynamic_amm::state::CurveType;
use dynamic_vault::program::DynamicVault as MeteoraVault;

#[derive(Accounts)]
pub struct InitChanAmmPool<'info> {
    /// Signer
    #[account(mut)]
    pub signer: Signer<'info>,
    /// Staking Pool Account
    #[account(
        mut,
        constraint = staking.quote_amm_pool.key() != system_program.key(),
        constraint = staking.chan_amm_pool.key() == system_program.key(),
        constraint = staking.is_active == false,
        seeds = [StakingPool::POOL_PREFIX, meme_mint.key().as_ref()],
        bump
    )]
    pub staking: Box<Account<'info, StakingPool>>,
    /// Staking Pool Signer
    /// CHECK: live phase pda signer
    #[account(mut, seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_pool_signer_pda: AccountInfo<'info>,
    #[account(
        mut,
        constraint = staking.quote_vault == staking_quote_vault.key()
    )]
    /// Staking Pool Chan vault
    pub staking_quote_vault: Box<Account<'info, TokenAccount>>,
    /// Staking Pool Meme vault
    #[account(
        mut,
        constraint = staking.meme_vault == staking_meme_vault.key()
    )]
    pub staking_meme_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = staking.chan_vault == staking_chan_vault.key()
    )]
    /// Staking Pool Chan vault
    pub staking_chan_vault: Box<Account<'info, TokenAccount>>,
    /// Mint Account for Meme
    #[account(
        constraint = staking_meme_vault.mint == meme_mint.key()
    )]
    pub meme_mint: Box<Account<'info, Mint>>,
    #[account(
        constraint = CHAN_MINT == chan_mint.key()
    )]
    /// Mint Account for Quote
    pub chan_mint: Box<Account<'info, Mint>>,
    // Chanswap
    pub chan_swap: Box<Account<'info, ChanSwap>>,
    /// CHECK: chan swap pda signer
    #[account(mut, seeds = [ChanSwap::SIGNER_PDA_PREFIX.as_bytes()], bump)]
    pub chan_swap_signer_pda: AccountInfo<'info>,
    #[account(mut)]
    pub chan_swap_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::authority = FEE_KEY)]
    pub fee_quote_vault: Box<Account<'info, TokenAccount>>,
    // Meteora Amm Program accounts
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: meteora cpi account
    pub fee_owner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub payer_pool_lp: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub amm_pool: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub mint_metadata: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub a_token_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub a_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub a_vault_lp: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub a_vault_lp_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub b_token_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub b_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub b_vault_lp: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub b_vault_lp_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub admin_token_a_fee: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub admin_token_b_fee: AccountInfo<'info>,

    // Lock
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub lock_escrow: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub escrow_vault: AccountInfo<'info>,

    // Sysvars
    pub rent: Sysvar<'info, Rent>,
    // Programs
    pub metadata_program: Program<'info, Metadata>,
    pub ata_program: Program<'info, AssociatedToken>,
    pub amm_program: Program<'info, MeteoraAmm>,
    pub vault_program: Program<'info, MeteoraVault>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitChanAmmPool<'info> {
    fn swap_tokens(
        &self,
        staking_signer_seeds: &[&[&[u8]]],
        swap_signer_seeds: &[&[&[u8]]],
        quote_amount: u64,
        chan_amount: u64,
    ) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.staking_quote_vault.to_account_info(),
            to: self.fee_quote_vault.to_account_info(),
            authority: self.staking_pool_signer_pda.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                staking_signer_seeds,
            ),
            quote_amount,
        )?;

        let cpi_accounts = Transfer {
            from: self.chan_swap_vault.to_account_info(),
            to: self.staking_chan_vault.to_account_info(),
            authority: self.chan_swap_signer_pda.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                cpi_accounts,
                swap_signer_seeds,
            ),
            chan_amount,
        )
    }

    fn create_pool(
        &self,
        seeds: &[&[&[u8]]],
        trade_fee_bps: u64,
        token_a_amount: u64,
        token_b_amount: u64,
    ) -> Result<()> {
        let program = self.amm_program.to_account_info();
        let cpi = dynamic_amm::cpi::accounts::InitializePermissionlessPoolWithFeeTier {
            lp_mint: self.lp_mint.to_account_info(),
            payer: self.staking_pool_signer_pda.to_account_info(),
            rent: self.rent.to_account_info(),
            fee_owner: self.fee_owner.to_account_info(),
            pool: self.amm_pool.to_account_info(),
            mint_metadata: self.mint_metadata.to_account_info(),
            a_token_vault: self.a_token_vault.to_account_info(),
            a_vault: self.a_vault.to_account_info(),
            a_vault_lp: self.a_vault_lp.to_account_info(),
            a_vault_lp_mint: self.a_vault_lp_mint.to_account_info(),
            token_a_mint: self.meme_mint.to_account_info(),
            token_b_mint: self.chan_mint.to_account_info(),
            b_token_vault: self.b_token_vault.to_account_info(),
            b_vault: self.b_vault.to_account_info(),
            b_vault_lp: self.b_vault_lp.to_account_info(),
            b_vault_lp_mint: self.b_vault_lp_mint.to_account_info(),
            admin_token_a_fee: self.admin_token_a_fee.to_account_info(),
            admin_token_b_fee: self.admin_token_b_fee.to_account_info(),
            payer_token_a: self.staking_meme_vault.to_account_info(),
            payer_token_b: self.staking_chan_vault.to_account_info(),
            payer_pool_lp: self.payer_pool_lp.to_account_info(),
            metadata_program: self.metadata_program.to_account_info(),
            associated_token_program: self.ata_program.to_account_info(),
            vault_program: self.vault_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(program, cpi, seeds);
        dynamic_amm::cpi::initialize_permissionless_pool_with_fee_tier(
            cpi_ctx,
            CurveType::ConstantProduct,
            trade_fee_bps,
            token_a_amount,
            token_b_amount,
        )
    }

    fn create_lock_escrow(&self, seeds: &[&[&[u8]]]) -> Result<()> {
        let program = self.amm_program.to_account_info();
        let cpi = dynamic_amm::cpi::accounts::CreateLockEscrow {
            lock_escrow: self.lock_escrow.to_account_info(),
            pool: self.amm_pool.to_account_info(),
            owner: self.staking_pool_signer_pda.to_account_info(),
            lp_mint: self.lp_mint.to_account_info(),
            payer: self.staking_pool_signer_pda.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(program, cpi, seeds);
        dynamic_amm::cpi::create_lock_escrow(cpi_ctx)
    }

    fn create_escrow_vault(&self) -> Result<()> {
        let program = self.ata_program.to_account_info();
        let cpi = anchor_spl::associated_token::Create {
            associated_token: self.escrow_vault.to_account_info(),
            authority: self.lock_escrow.to_account_info(),
            payer: self.signer.to_account_info(),
            mint: self.lp_mint.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(program, cpi);
        anchor_spl::associated_token::create_idempotent(cpi_ctx)
    }

    fn lock(&self, lp_amount: u64, seeds: &[&[&[u8]]]) -> Result<()> {
        let program = self.amm_program.to_account_info();
        let cpi = dynamic_amm::cpi::accounts::Lock {
            lock_escrow: self.lock_escrow.to_account_info(),
            pool: self.amm_pool.to_account_info(),
            owner: self.staking_pool_signer_pda.to_account_info(),
            lp_mint: self.lp_mint.to_account_info(),
            a_vault: self.a_vault.to_account_info(),
            a_vault_lp: self.a_vault_lp.to_account_info(),
            a_vault_lp_mint: self.a_vault_lp_mint.to_account_info(),
            b_vault: self.b_vault.to_account_info(),
            b_vault_lp: self.b_vault_lp.to_account_info(),
            b_vault_lp_mint: self.b_vault_lp_mint.to_account_info(),
            escrow_vault: self.escrow_vault.to_account_info(),
            source_tokens: self.payer_pool_lp.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(program, cpi, seeds);
        dynamic_amm::cpi::lock(cpi_ctx, lp_amount)
    }
}

pub fn handle(ctx: Context<InitChanAmmPool>) -> Result<()> {
    let accs = ctx.accounts;

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_pool_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    let swap_signer_seeds: &[&[&[u8]]] = &[&[
        ChanSwap::SIGNER_PDA_PREFIX.as_bytes(),
        &[ctx.bumps.chan_swap_signer_pda],
    ]];

    // 1. Swap SOL to CHAN
    let quote_amt = accs.staking_quote_vault.amount;
    let chan_amt = quote_amt
        .mul_div_floor(
            accs.chan_swap.chan_sol_price_num,
            accs.chan_swap.chan_sol_price_denom,
        )
        .unwrap();
    msg!("swapped {} to {} chan tokens", quote_amt, chan_amt);
    accs.swap_tokens(staking_signer_seeds, swap_signer_seeds, quote_amt, chan_amt)?;

    // 2. Get supply values for the new pool
    accs.staking_chan_vault.reload()?;

    let meme_supply = accs.staking_meme_vault.amount - accs.staking.stakes_total;
    let chan_supply = accs.staking_chan_vault.amount;

    msg!("3");
    // 3. Initialize pool & Add liquidity to the pool
    let trade_fee_bps = 100u64;
    accs.create_pool(
        staking_signer_seeds,
        trade_fee_bps,
        meme_supply,
        chan_supply,
    )?;

    msg!("4");
    // 4. Create lock
    accs.create_lock_escrow(staking_signer_seeds)?;

    msg!("5.1");
    // 5.1 Create escrow ata
    accs.create_escrow_vault()?;

    msg!("5.2");
    // 5.2 Lock tokens

    let lp_amount = {
        let account_data = accs.payer_pool_lp.try_borrow_data()?;
        let mut account_data_slice: &[u8] = &account_data;
        let token_acc = TokenAccount::try_deserialize(&mut account_data_slice)?;
        token_acc.amount
    };
    msg!("5.3");
    accs.lock(lp_amount, staking_signer_seeds)?;

    msg!("6");
    // 6. Setup staking
    // Add LP vault and mint to staking pool
    accs.staking.chan_amm_pool = accs.amm_pool.key();
    accs.staking.is_active = true;

    Ok(())
}
