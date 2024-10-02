use crate::consts::{CHAN_MINT, SWAP_FEE_KEY};
use crate::libraries::MulDiv;
use crate::models::chan_swap::ChanSwap;
use crate::models::staking::StakingPool;
use aldrin_amm::program::MmFarmingPool as AldrinAmm;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

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
    #[account(mut, token::authority = SWAP_FEE_KEY)]
    pub fee_quote_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK: aldrin pool account
    pub amm_pool: AccountInfo<'info>,
    // #[account(constraint = pool_mint.mint_authority == COption::Some(* pool_signer.key),
    // constraint = pool_mint.supply == 0,
    // constraint = pool_mint.freeze_authority == COption::None)]
    amm_pool_mint: Account<'info, Mint>,
    // #[account(constraint = lp_token_freeze_vault.owner == * pool_signer.key,
    // constraint = lp_token_freeze_vault.close_authority == COption::None)]
    lp_token_freeze_vault: Account<'info, TokenAccount>,
    // #[account(constraint = base_token_vault.owner == * pool_signer.key,
    // constraint = base_token_vault.delegate == COption::None,
    // constraint = base_token_vault.close_authority == COption::None)]
    amm_base_token_vault: Account<'info, TokenAccount>,
    // #[account(constraint = quote_token_vault.owner == * pool_signer.key,
    // constraint = quote_token_vault.delegate == COption::None,
    // constraint = quote_token_vault.close_authority == COption::None)]
    amm_quote_token_vault: Account<'info, TokenAccount>,
    /// CHECK: aldrin
    amm_pool_signer: AccountInfo<'info>,
    /// CHECK: aldrin
    //#[account(constraint = *pool_authority.key == pool_authority::ID)]
    amm_pool_authority: AccountInfo<'info>,
    // #[account(constraint = fee_base_account.owner == fee_owner::ID)]
    amm_fee_base_account: Account<'info, TokenAccount>,
    // #[account(constraint = fee_quote_account.owner == fee_owner::ID)]
    amm_fee_quote_account: Account<'info, TokenAccount>,
    // #[account(constraint = fee_pool_token_account.owner == fee_owner::ID,
    // constraint = fee_pool_token_account.close_authority == COption::Some(* pool_signer.key))]
    amm_fee_pool_token_account: Account<'info, TokenAccount>,
    /// CHECK: aldrin
    #[account(mut)]
    amm_curve: AccountInfo<'info>,

    // Sysvars
    pub rent: Sysvar<'info, Rent>,
    // Programs
    pub aldrin_amm_program: Program<'info, AldrinAmm>,
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

    fn create_pool(&self, seeds: &[&[&[u8]]], signer_nonce: u8) -> Result<()> {
        let program = self.aldrin_amm_program.to_account_info();
        let cpi = aldrin_amm::cpi::accounts::InitializeConstProductCurve {
            curve: self.amm_curve.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(program, cpi);
        aldrin_amm::cpi::initialize_const_product_curve(cpi_ctx)?;

        let program = self.aldrin_amm_program.to_account_info();
        let cpi = aldrin_amm::cpi::accounts::Initialize {
            pool: self.amm_pool.to_account_info(),
            pool_signer: self.amm_pool_signer.to_account_info(),
            pool_authority: self.staking_pool_signer_pda.to_account_info(),
            fee_base_account: self.amm_fee_base_account.to_account_info(),
            fee_quote_account: self.amm_fee_quote_account.to_account_info(),
            initializer: self.staking_pool_signer_pda.to_account_info(),
            pool_mint: self.amm_pool_mint.to_account_info(),
            curve: self.amm_curve.to_account_info(),
            base_token_vault: self.amm_base_token_vault.to_account_info(),
            quote_token_vault: self.amm_quote_token_vault.to_account_info(),
            lp_token_freeze_vault: self.lp_token_freeze_vault.to_account_info(),
            fee_pool_token_account: self.amm_fee_pool_token_account.to_account_info(),
            token_program: self.token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(program, cpi, seeds);
        aldrin_amm::cpi::initialize(cpi_ctx, signer_nonce, 0)
    }
}

pub fn handle(ctx: Context<InitChanAmmPool>, signer_nonce: u8) -> Result<()> {
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
    accs.create_pool(staking_signer_seeds, signer_nonce)?;

    // msg!("4");
    // // 4. Create lock
    // accs.create_lock_escrow(staking_signer_seeds)?;

    // msg!("5.1");
    // // 5.1 Create escrow ata
    // accs.create_escrow_vault()?;

    // msg!("5.2");
    // // 5.2 Lock tokens
    //
    // let lp_amount = {
    //     let account_data = accs.payer_pool_lp.try_borrow_data()?;
    //     let mut account_data_slice: &[u8] = &account_data;
    //     let token_acc = TokenAccount::try_deserialize(&mut account_data_slice)?;
    //     token_acc.amount
    // };
    // msg!("5.3");
    // accs.lock(lp_amount, staking_signer_seeds)?;

    msg!("6");
    // 6. Setup staking
    // Add LP vault and mint to staking pool
    accs.staking.chan_amm_pool = accs.amm_pool.key();
    accs.staking.is_active = true;

    Ok(())
}
