use crate::consts::{MAX_TICKET_TOKENS, MEME_TOKEN_DECIMALS};
use crate::models::staking::StakingPool;
use crate::models::OpenBook;
use crate::raydium;
use crate::raydium::RaydiumAmm;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
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
        constraint = staking.quote_vault == pool_quote_vault.key()
    )]
    //
    /// Staking Pool WSOL vault
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
    pub user_destination_lp_token_ata: AccountInfo<'info>,
    //
    // Sysvars
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    //
    // Programs
    pub raydium_program: Program<'info, RaydiumAmm>,
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
        seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let instruction = raydium::initialize2(
            &self.raydium_program.key(),
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
            &self.raydium_lp_mint.key(),     // lp mint
            &self.meme_mint.key(),           // coin mint
            &self.quote_mint.key(),          // pc_mint
            &self.raydium_meme_vault.key(),  // coin_vault
            &self.raydium_quote_vault.key(), // pc_vault
            &self.target_orders.key(),
            &self.amm_config.key(),
            &self.fee_destination_info.key(),
            &self.market_program_id.key(),
            &self.market_account.key(),
            &self.staking_pool_signer_pda.key(), // user/signer
            &self.pool_meme_vault.key(),
            &self.pool_quote_vault.key(),
            &self.user_destination_lp_token_ata.key(),
        );
        solana_program::program::invoke_signed(
            &instruction,
            &[
                self.token_program.to_account_info().clone(), // 0. `[]` Spl Token program id
                self.ata_program.to_account_info().clone(),   // 1. `[]` Associated Token program id
                self.system_program.to_account_info().clone(), // 2. `[]` Sys program id
                self.rent.to_account_info().clone(),          // 3. `[]` Rent program id
                self.raydium_amm.to_account_info().clone(), // 4. `[writable]` New AMM Account to create.
                self.raydium_amm_authority.to_account_info().clone(), // 5. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
                self.open_orders.to_account_info().clone(), // 6. `[writable]` AMM open orders Account
                self.raydium_lp_mint.to_account_info().clone(), // 7. `[writable]` AMM lp mint Account
                self.meme_mint.to_account_info().clone(),       // 8. `[]` AMM coin mint Account
                self.quote_mint.to_account_info().clone(),      // 9. `[]` AMM pc mint Account
                self.raydium_meme_vault.to_account_info().clone(), // 10. `[writable]` AMM coin vault Account. Must be non zero, owned by $authority.
                self.raydium_quote_vault.to_account_info().clone(), // 11. `[writable]` AMM pc vault Account. Must be non zero, owned by $authority.
                self.target_orders.to_account_info().clone(), // 12. `[writable]` AMM target orders Account. To store plan orders informations.
                self.amm_config.to_account_info().clone(), // 13. `[]` AMM config Account, derived from `find_program_address(&[&&AMM_CONFIG_SEED])`.
                self.fee_destination_info.to_account_info().clone(), // 14. `[]` AMM create pool fee destination Account
                self.market_program_id.to_account_info().clone(),    // 15. `[]` Market program id
                self.market_account.to_account_info().clone(), // 16. `[writable]` Market Account. Market program is the owner.
                self.staking_pool_signer_pda.to_account_info().clone(), // 17. `[writable, singer]` User wallet Account
                self.pool_meme_vault.to_account_info().clone(), // 18. `[]` User token coin Account
                self.pool_quote_vault.to_account_info().clone(), // 19. '[]` User token pc Account
                self.user_destination_lp_token_ata.to_account_info().clone(), // 20. `[writable]` User destination lp token ATA Account
            ],
            seeds,
        )?;

        Ok(())
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
    let quote_supply = accs.pool_quote_vault.amount;

    // 2. Split MEME balance amounts into 80/20
    let meme_supply = accs.pool_meme_vault.amount;
    let meme_supply_80 = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;

    let amm_meme_balance = meme_supply.checked_sub(meme_supply_80).unwrap();

    msg!("3");
    // 3. Initialize pool & Add liquidity to the pool
    accs.create_raydium_pool(
        nonce,
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

    Ok(())
}
