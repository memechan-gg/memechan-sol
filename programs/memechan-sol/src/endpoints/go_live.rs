use crate::consts::{MAX_TICKET_TOKENS, MEME_TOKEN_DECIMALS};
use crate::models::staking::StakingPool;
use crate::models::OpenBook;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use raydium_cp_swap::program::RaydiumCpSwap as Raydium;

#[derive(Accounts)]
pub struct GoLive<'info> {
    /// Signer
    #[account(mut)]
    pub signer: Signer<'info>,
    //
    //
    //
    //
    //
    // ===== Staking Pool Accounts =====
    //
    /// Staking Pool Account
    #[account(
        mut,
        constraint = staking.raydium_amm.key() == system_program.key(),
        seeds = [StakingPool::POOL_PREFIX, meme_mint.key().as_ref()],
        bump
    )]
    pub staking: Box<Account<'info, StakingPool>>,
    //
    /// Staking Pool Signer
    /// CHECK: live phase pda signer
    #[account(mut, seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_pool_signer_pda: AccountInfo<'info>,
    //
    /// Staking Pool Meme vault
    #[account(
        mut,
        constraint = staking.meme_vault == pool_meme_vault.key()
    )]
    pub pool_meme_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = staking.quote_vault == pool_quote_vault.key()
    )]
    /// Staking Pool Quote vault
    pub pool_quote_vault: Box<Account<'info, TokenAccount>>,
    //
    //
    //
    // ===== Memechan Mint Accounts =====
    //
    /// Mint Account for Meme
    #[account(
        constraint = pool_meme_vault.mint == meme_mint.key()
    )]
    pub meme_mint: Box<Account<'info, Mint>>,
    //
    /// Mint Account for WSOL
    pub quote_mint: Box<Account<'info, Mint>>,
    //
    //
    //
    // ===== Raydium Accounts =====
    //
    /// Raydium AMM Account
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub raydium_amm: UncheckedAccount<'info>,
    // pub raydium_amm: AccountLoader<'info, AmmInfo>,
    /// Raydium AMM Signer
    /// CHECK: Raydium signer, checks done in cpi call to raydium
    #[account(mut)]
    pub raydium_amm_authority: AccountInfo<'info>,
    /// Raydium LP MinT
    /// CHECK: live phase pda signer
    #[account(mut)]
    pub raydium_lp_mint: UncheckedAccount<'info>,
    /// Raydium LP Token Account
    // #[account(mut)]
    // pub pool_lp_wallet: Box<Account<'info, TokenAccount>>,
    /// Raydium Meme Token Account
    #[account(mut)]
    /// CHECK: Checks done in cpi call to raydium
    pub raydium_meme_vault: UncheckedAccount<'info>,
    /// Raydium WSOL Token Account
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub raydium_quote_vault: UncheckedAccount<'info>,
    /// CHECK: Checks done in cpi call to raydium
    pub amm_config: UncheckedAccount<'info>,
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub fee_destination_info: AccountInfo<'info>,
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub observation_state: AccountInfo<'info>,
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub user_destination_lp_token_ata: AccountInfo<'info>,
    //
    // Sysvars
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    //
    // Programs
    pub raydium_program: Program<'info, Raydium>,
    /// CHECK: Checks done in cpi call to raydium
    pub ata_program: Program<'info, AssociatedToken>,
    // Checked by raydium account
    pub market_program_id: Program<'info, OpenBook>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> GoLive<'info> {
    fn create_raydium_pool(
        &self,
        open_time: u64,
        init_pc_amount: u64,
        init_coin_amount: u64,
        seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let program = self.raydium_program.to_account_info();
        let cpi  = raydium_cp_swap::cpi::accounts::Initialize {
            creator: self.staking_pool_signer_pda.to_account_info(),
            amm_config: self.amm_config.to_account_info(),
            authority: self.raydium_amm_authority.to_account_info(),
            pool_state: self.raydium_amm.to_account_info(),
            token_0_mint: self.meme_mint.to_account_info(),
            token_1_mint: self.quote_mint.to_account_info(),
            lp_mint: self.raydium_lp_mint.to_account_info(),
            creator_token_0: self.pool_meme_vault.to_account_info(),
            creator_token_1: self.pool_quote_vault.to_account_info(),
            creator_lp_token: self.user_destination_lp_token_ata.to_account_info(),
            token_0_vault: self.raydium_meme_vault.to_account_info(),
            token_1_vault: self.raydium_quote_vault.to_account_info(),
            create_pool_fee: self.fee_destination_info.to_account_info(),
            observation_state: self.observation_state.to_account_info(),
            token_program: self.token_program.to_account_info(),
            token_0_program: self.token_program.to_account_info(),
            token_1_program: self.token_program.to_account_info(),
            associated_token_program: self.ata_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(program, cpi, seeds);
        raydium_cp_swap::cpi::initialize(cpi_ctx, init_coin_amount, init_pc_amount, open_time)
    }
}

pub fn handle<'info>(ctx: Context<'_, '_, '_, 'info, GoLive<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_pool_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    // 1. Get Sol Supply
    let quote_supply = accs.pool_quote_vault.amount;

    // 2. Split MEME balance amounts into 80/20
    let meme_supply = accs.pool_meme_vault.amount;
    let meme_supply_80 = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;

    let amm_meme_balance = meme_supply.checked_sub(meme_supply_80).unwrap();

    msg!("3");
    // 3. Initialize pool & Add liquidity to the pool
    accs.create_raydium_pool(
        accs.clock.unix_timestamp as u64, // open time
        quote_supply,                     // init_pc_amount
        amm_meme_balance,                 // init_coin_amount
        staking_signer_seeds,
    )?;

    msg!("4");
    // 4. Setup staking
    // Add LP vault and mint to staking pool
    accs.staking.lp_mint = accs.raydium_lp_mint.key();
    accs.staking.lp_vault = accs.user_destination_lp_token_ata.key();
    accs.staking.raydium_amm = accs.raydium_amm.key();

    Ok(())
}
