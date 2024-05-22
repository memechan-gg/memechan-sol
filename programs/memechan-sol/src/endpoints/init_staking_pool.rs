use crate::consts::{MAX_TICKET_TOKENS, MEME_TOKEN_DECIMALS, SLERF_MINT};
use crate::err;
use crate::err::AmmError;
use crate::libraries::MulDiv;
use crate::models::bound::BoundPool;
use crate::models::fees::{LAUNCH_FEE, PRECISION};
use crate::models::staked_lp::MemeTicket;
use crate::models::staking::StakingPool;
use crate::models::OpenBook;
use crate::{admin, vesting};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct InitStakingPool<'info> {
    /// Signer
    #[account(mut)]
    pub signer: Signer<'info>,
    //
    //
    // ===== Bonding Pool =====
    //
    /// Bonding Pool account
    #[account(
        mut,
        has_one = admin_vault_quote,
        constraint = pool.locked
    )]
    pub pool: Box<Account<'info, BoundPool>>,
    /// CHECK: bound-curve phase pda signer
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub bound_pool_signer_pda: AccountInfo<'info>,
    /// Bonding Pool Meme vault
    #[account(
        mut,
        constraint = pool.meme_reserve.vault == pool_meme_vault.key()
    )]
    pub pool_meme_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = pool.quote_reserve.vault == pool_quote_vault.key()
    )]
    /// Bonding Pool WSOL vault
    pub pool_quote_vault: Box<Account<'info, TokenAccount>>,
    /// Bonding Pool Admin Vault
    #[account(
        mut,
        constraint = pool.admin_vault_quote == admin_vault_quote.key()
    )]
    pub admin_vault_quote: Box<Account<'info, TokenAccount>>,
    //
    //
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
    /// Mint Account for WSOL
    #[account(
        constraint = quote_mint.key() == SLERF_MINT
            @ err::acc("Quote mint should be native SLERF mint")
    )]
    pub quote_mint: Box<Account<'info, Mint>>,
    //
    //
    //
    //
    //
    // ===== Staking Pool Accounts =====
    //
    /// Staking Pool Account
    #[account(
        init,
        payer = signer,
        space = StakingPool::space(),
        seeds = [StakingPool::POOL_PREFIX, meme_mint.key().as_ref()],
        bump
    )]
    pub staking: Box<Account<'info, StakingPool>>,
    //
    /// Staking Pool Signer
    /// CHECK: live phase pda signer
    #[account(mut, seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_pool_signer_pda: AccountInfo<'info>,

    #[account(
        mut,
        constraint = staking_meme_vault.owner == staking_pool_signer_pda.key()
    )]
    pub staking_meme_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = staking_quote_vault.owner == staking_pool_signer_pda.key()
    )]
    /// Bonding Pool WSOL vault
    pub staking_quote_vault: Box<Account<'info, TokenAccount>>,
    //
    /// Meme Ticket Account of Admin
    #[account(
        init,
        payer = signer,
        space = MemeTicket::space(),
        seeds = [MemeTicket::ADMIN_TICKET_PREFIX, staking.key().as_ref()],
        bump
    )]
    pub meme_ticket: Box<Account<'info, MemeTicket>>,
    //
    //
    //
    //
    //
    // Sysvars
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,

    // Programs
    /// CHECK: Checks done in cpi call to raydium
    pub ata_program: Program<'info, AssociatedToken>,
    // Checked by raydium account
    pub market_program_id: Program<'info, OpenBook>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitStakingPool<'info> {
    fn token_transfer_meme_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.pool_meme_vault.to_account_info(),
            to: self.staking_meme_vault.to_account_info(),
            authority: self.bound_pool_signer_pda.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn token_transfer_quote_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.pool_quote_vault.to_account_info(),
            to: self.staking_quote_vault.to_account_info(),
            authority: self.bound_pool_signer_pda.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_admin_fee_sol(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.pool_quote_vault.to_account_info(),
            to: self.admin_vault_quote.to_account_info(),
            authority: self.bound_pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle<'info>(ctx: Context<'_, '_, '_, 'info, InitStakingPool<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    let bp_seeds = &[
        BoundPool::SIGNER_PDA_PREFIX,
        &accs.pool.key().to_bytes()[..],
        &[ctx.bumps.bound_pool_signer_pda],
    ];

    let bp_signer_seeds = &[&bp_seeds[..]];

    // 0. Create admin ticket + withdraw sol fees
    msg!("0");
    let meme_ticket = &mut accs.meme_ticket;

    meme_ticket.setup(accs.pool.key(), admin::id(), accs.pool.admin_fees_meme);

    if accs.pool.admin_fees_quote != 0 {
        token::transfer(
            accs.send_admin_fee_sol().with_signer(bp_signer_seeds),
            accs.pool.admin_fees_quote,
        )
        .unwrap();
    };

    // 1. Verify if we reached the threshold of SUI amount raised
    msg!("1");
    accs.pool_quote_vault.reload().unwrap();
    let quote_supply = accs.pool_quote_vault.amount;
    let target_token_amt = accs.pool.config.gamma_s;
    let quote_decimals = accs.pool.config.decimals.quote as u32;
    if quote_supply != target_token_amt * 10_u64.checked_pow(quote_decimals).unwrap() {
        return Err(error!(AmmError::InvariantViolation));
    }

    // 2. Collect live fees
    msg!("2");
    let live_fee_amt = quote_supply.mul_div_ceil(LAUNCH_FEE, PRECISION).unwrap();
    token::transfer(
        accs.send_admin_fee_sol().with_signer(bp_signer_seeds),
        live_fee_amt,
    )
    .unwrap();

    accs.pool_quote_vault.reload().unwrap();

    // 3. Transfer pool_quote_vault
    msg!("3");
    msg!(
        "Amount of wSOL to transfer {:?}",
        accs.pool_quote_vault.amount
    );

    token::transfer(
        accs.token_transfer_quote_ctx().with_signer(bp_signer_seeds),
        accs.pool_quote_vault.amount,
    )
    .unwrap();

    msg!(
        "Amount of Meme to transfer {:?}",
        accs.pool_meme_vault.amount
    );

    token::transfer(
        accs.token_transfer_meme_ctx().with_signer(bp_signer_seeds),
        accs.pool_meme_vault.amount,
    )
    .unwrap();

    // 4. Setup new staking account
    msg!("4");
    let staking = &mut accs.staking;

    staking.meme_vault = accs.staking_meme_vault.key();
    staking.meme_mint = accs.meme_mint.key();
    staking.quote_vault = accs.staking_quote_vault.key();
    staking.stakes_total = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;
    staking.vesting_config = vesting::default_config();
    staking.fees_x_total = 0;
    staking.fees_y_total = 0;
    staking.lp_tokens_withdrawn = 0;
    staking.pool = accs.pool.key();

    msg!("5");

    Ok(())
}
