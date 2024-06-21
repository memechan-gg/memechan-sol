use crate::models::staking::StakingPool;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use dynamic_amm::program::DynamicAmm as MeteoraAmm;
use dynamic_vault::program::DynamicVault as MeteoraVault;

#[derive(Accounts)]
pub struct AddFees<'info> {
    #[account(
        mut,
        has_one = meme_vault,
        has_one = quote_vault,
        has_one = amm_pool
    )]
    pub staking: Account<'info, StakingPool>,
    #[account(mut)]
    pub meme_vault: Box<Account<'info, TokenAccount>>,
    pub meme_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub quote_vault: Box<Account<'info, TokenAccount>>,
    pub quote_mint: Box<Account<'info, Mint>>,
    /// CHECK: pda
    #[account(mut, seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_signer_pda: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub amm_pool: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub lp_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub a_token_vault: AccountInfo<'info>,
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
    pub b_token_vault: AccountInfo<'info>,
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
    pub lock_escrow: Box<Account<'info, dynamic_amm::state::LockEscrow>>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub escrow_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub source_tokens: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    // Programs
    pub token_program: Program<'info, Token>,
    pub amm_program: Program<'info, MeteoraAmm>,
    pub vault_program: Program<'info, MeteoraVault>,
    /// CHECK: Check is done by Raydium inside CPI
    pub memo_program: AccountInfo<'info>,
}

impl<'info> AddFees<'info> {
    pub fn collect_fees(&self, amount: u64, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let program = self.amm_program.to_account_info();
        let mut cpi = dynamic_amm::cpi::accounts::ClaimFee {
            owner: self.staking_signer_pda.to_account_info(),
            lp_mint: self.lp_mint.to_account_info(),
            pool: self.amm_pool.to_account_info(),
            a_token_vault: self.a_token_vault.to_account_info(),
            a_vault: self.a_vault.to_account_info(),
            a_vault_lp: self.a_vault_lp.to_account_info(),
            a_vault_lp_mint: self.a_vault_lp_mint.to_account_info(),
            b_token_vault: self.b_token_vault.to_account_info(),
            b_vault: self.b_vault.to_account_info(),
            b_vault_lp: self.b_vault_lp.to_account_info(),
            b_vault_lp_mint: self.b_vault_lp_mint.to_account_info(),
            lock_escrow: self.lock_escrow.to_account_info(),
            escrow_vault: self.escrow_vault.to_account_info(),
            source_tokens: self.source_tokens.to_account_info(),
            user_a_token: self.meme_vault.to_account_info(),
            user_b_token: self.quote_vault.to_account_info(),
            vault_program: self.vault_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        if self.meme_mint.key() > self.quote_mint.key() {
            cpi.user_a_token = self.quote_vault.to_account_info();
            cpi.user_b_token = self.meme_vault.to_account_info();
        }

        let cpi_ctx = CpiContext::new_with_signer(program, cpi, signer_seeds);
        dynamic_amm::cpi::claim_fee(cpi_ctx, amount)
    }
}

pub fn handle<'info>(ctx: Context<'_, '_, '_, 'info, AddFees<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    let lp_tokens_total = accs.lock_escrow.total_locked_amount;

    let meme_vault_initial_amt = accs.meme_vault.amount;
    let quote_vault_initial_amt = accs.quote_vault.amount;

    let cpi_res = accs.collect_fees(lp_tokens_total, staking_signer_seeds);

    if cpi_res.is_err() {
        return Ok(());
    }

    accs.meme_vault.reload().unwrap();
    accs.quote_vault.reload().unwrap();

    msg!(
        "meme_vault_amt {} meme_vault_initial_amt {} quote_vault_amt {} quote_vault_initial_amt {}",
        accs.meme_vault.amount,
        meme_vault_initial_amt,
        accs.quote_vault.amount,
        quote_vault_initial_amt
    );

    let fees_x = accs
        .meme_vault
        .amount
        .checked_sub(meme_vault_initial_amt)
        .unwrap();
    let fees_y = accs
        .quote_vault
        .amount
        .checked_sub(quote_vault_initial_amt)
        .unwrap();

    let state = &mut accs.staking;

    state.fees_x_total += fees_x;
    state.fees_y_total += fees_y;

    Ok(())
}
