use crate::consts::{MAX_TICKET_TOKENS, MEME_TOKEN_DECIMALS, RAYDIUM_PROGRAM_ID};
use crate::models::staking::StakingPool;
use crate::models::OpenBook;
use crate::raydium::models::{AmmConfig, AmmInfo, MarketState, OpenOrders, TargetOrders};
use crate::{err, raydium};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::spl_token::native_mint;
use anchor_spl::token::{Mint, SetAuthority, Token, TokenAccount};

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
        constraint = staking.wsol_vault == pool_wsol_vault.key()
    )]
    //
    /// Staking Pool WSOL vault
    pub pool_wsol_vault: Box<Account<'info, TokenAccount>>,
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
    #[account(
        constraint = sol_mint.key() == native_mint::id()
            @ err::acc("sol mint should be native WSOL mint")
    )]
    pub sol_mint: Box<Account<'info, Mint>>,
    //
    //
    //
    // ===== OpenBook Accounts =====
    //
    /// Open Orders Account
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    /// Target Orders Account
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub target_orders: UncheckedAccount<'info>,
    /// Market Orders Account
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub market_account: UncheckedAccount<'info>,
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
    pub raydium_meme_vault: Box<Account<'info, TokenAccount>>,
    /// Raydium WSOL Token Account
    #[account(mut)]
    pub raydium_wsol_vault: Box<Account<'info, TokenAccount>>,
    /// CHECK: Checks done in cpi call to raydium
    pub amm_config: UncheckedAccount<'info>,
    /// CHECK: Checks done in cpi call to raydium
    pub fee_destination_info: AccountInfo<'info>,
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub user_destination_lp_token_ata: AccountInfo<'info>,
    //
    // Sysvars
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    //
    // Programs
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
        nonce: u8,
        open_time: u64,
        init_pc_amount: u64,
        init_coin_amount: u64,
    ) -> Result<()> {
        let instruction = raydium::initialize2(
            &RAYDIUM_PROGRAM_ID,
            // Params
            nonce,
            open_time,
            init_pc_amount,
            init_coin_amount,
            // Accounts
            &self.token_program.key(),
            &self.ata_program.key(),
            &self.system_program.key(),
            &self.rent.key(),
            &self.raydium_amm.key(),
            &self.raydium_amm_authority.key(),
            &self.open_orders.key(),
            &self.raydium_lp_mint.key(),    // lp mint
            &self.meme_mint.key(),          // coin mint
            &self.sol_mint.key(),           // pc_mint
            &self.raydium_meme_vault.key(), // coin_vault
            &self.raydium_wsol_vault.key(), // pc_vault
            &self.target_orders.key(),
            &self.amm_config.key(),
            &self.fee_destination_info.key(),
            &self.market_program_id.key(),
            &self.market_account.key(),
            &self.signer.key(), // user/signer
            &self.pool_meme_vault.key(),
            &self.pool_wsol_vault.key(),
            &self.user_destination_lp_token_ata.key(),
        );
        solana_program::program::invoke(
            &instruction,
            &[
                self.token_program.to_account_info().clone(),
                self.ata_program.to_account_info().clone(),
                self.system_program.to_account_info().clone(),
                self.rent.to_account_info().clone(),
                self.raydium_amm.to_account_info().clone(),
                self.raydium_amm_authority.to_account_info().clone(),
                self.open_orders.to_account_info().clone(),
                self.raydium_lp_mint.to_account_info().clone(),
                self.meme_mint.to_account_info().clone(),
                self.sol_mint.to_account_info().clone(),
                self.raydium_meme_vault.to_account_info().clone(),
                self.raydium_wsol_vault.to_account_info().clone(),
                self.target_orders.to_account_info().clone(),
                self.amm_config.to_account_info().clone(),
                self.fee_destination_info.to_account_info().clone(),
                self.market_program_id.to_account_info().clone(),
                self.market_account.to_account_info().clone(),
                self.signer.to_account_info().clone(),
                self.pool_meme_vault.to_account_info().clone(),
                self.pool_wsol_vault.to_account_info().clone(),
                self.user_destination_lp_token_ata.to_account_info().clone(),
            ],
        )?;

        Ok(())
    }

    pub fn set_lp_wallet_authority(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        todo!()
        // let cpi_accounts = SetAuthority {
        //     current_authority: self.bound_pool_signer_pda.to_account_info(),
        //     account_or_mint: self.pool_meme_vault.to_account_info(), // this should be LP vault and not meme vault
        // };

        // let cpi_program = self.token_program.to_account_info();
        // CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle<'info>(ctx: Context<'_, '_, '_, 'info, GoLive<'info>>, nonce: u8) -> Result<()> {
    let accs = ctx.accounts;

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_pool_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    // 1. Get Sol Supply
    msg!("1");
    let sol_supply = accs.pool_wsol_vault.amount;

    // 2. Split MEME balance amounts into 80/20
    msg!("2");
    let meme_supply = accs.pool_meme_vault.amount;
    let meme_supply_80 = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;

    let amm_meme_balance = meme_supply.checked_sub(meme_supply_80).unwrap();

    // 3. Initialize pool & Add liquidity to the pool
    msg!("3");
    accs.create_raydium_pool(
        nonce,
        accs.clock.unix_timestamp as u64, // open time
        sol_supply,                       // init_pc_amount
        amm_meme_balance,                 // init_coin_amount
    )?;

    msg!("4");

    Ok(())
}
