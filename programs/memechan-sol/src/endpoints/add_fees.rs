use crate::{
    models::staking::{lp_tokens_to_burn, StakingPool},
};

use raydium_cp_swap::{self, states::PoolState};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token_2022::Token2022;
use raydium_cp_swap::program::RaydiumCpSwap as Raydium;

#[derive(Accounts)]
pub struct AddFees<'info> {
    #[account(
        mut,
        has_one = meme_vault,
        has_one = quote_vault
    )]
    pub staking: Account<'info, StakingPool>,
    #[account(mut)]
    pub meme_vault: Account<'info, TokenAccount>,
    pub meme_mint: Account<'info, Mint>,

    #[account(mut)]
    pub quote_vault: Account<'info, TokenAccount>,
    pub quote_mint: Account<'info, Mint>,
    /// CHECK: pda
    #[account(mut, seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_signer_pda: AccountInfo<'info>,
    #[account(
        mut,
        constraint = staking_lp_wallet.key() == staking.lp_vault
    )]
    pub staking_lp_wallet: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub signer: Signer<'info>,

    // raydium
    /// CHECK: Checks are done by us and in cpi call to raydium
    // Raydium
    #[account(mut)]
    pub raydium_amm: AccountLoader<'info, PoolState>,
    /// CHECK: Checks done in cpi call to raydium
    pub raydium_amm_authority: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_meme_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub raydium_quote_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = raydium_lp_mint.key() == staking.lp_mint
    )]
    pub raydium_lp_mint: Account<'info, Mint>,
    // Programs
    pub token_program: Program<'info, Token>,
    pub token_program22: Program<'info, Token2022>,
    pub raydium_program: Program<'info, Raydium>,
    /// CHECK: Check is done by Raydium inside CPI
    pub memo_program: AccountInfo<'info>
}

impl<'info> AddFees<'info> {
    pub fn redeem_liquidity(&self, amount: u64, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let program = self.raydium_program.to_account_info();
        let cpi  = raydium_cp_swap::cpi::accounts::Withdraw {
            owner: self.staking_signer_pda.to_account_info(),
            authority: self.raydium_amm_authority.to_account_info(),
            pool_state: self.raydium_amm.to_account_info(),
            owner_lp_token: self.staking_lp_wallet.to_account_info(),
            token_0_account: self.meme_vault.to_account_info(),
            token_1_account: self.quote_vault.to_account_info(),
            token_0_vault: self.raydium_meme_vault.to_account_info(),
            token_1_vault:self.raydium_quote_vault.to_account_info(),
            token_program: self.token_program.to_account_info(),
            token_program_2022: self.token_program22.to_account_info(),
            vault_0_mint: self.meme_mint.to_account_info(),
            vault_1_mint: self.quote_mint.to_account_info(),
            lp_mint: self.raydium_lp_mint.to_account_info(),
            memo_program: self.memo_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(program, cpi, signer_seeds);
        raydium_cp_swap::cpi::withdraw(cpi_ctx, amount, 0, 0)
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

    let meme_vault_initial_amt = accs.meme_vault.amount;
    let quote_vault_initial_amt = accs.quote_vault.amount;

    let amm_info = &accs.raydium_amm.load().unwrap();

    let cumulated_fees_meme = amm_info.fund_fees_token_0;
    let cumulated_fees_quote = amm_info.fund_fees_token_1;

    let fee_ratio = accs.staking.compute_fee_ratio(
        accs.raydium_meme_vault.amount,
        cumulated_fees_meme,
        accs.raydium_quote_vault.amount,
        cumulated_fees_quote,
    )?;

    let lp_tokens_owned = accs.staking_lp_wallet.amount;

    let lp_tokens_to_burn = lp_tokens_to_burn(fee_ratio, lp_tokens_owned)?;

    if lp_tokens_to_burn == 0 {
        msg!("No fees to collect");
        return Ok(());
    }

    accs.redeem_liquidity(lp_tokens_to_burn, staking_signer_seeds)?;

    accs.meme_vault.reload().unwrap();
    accs.quote_vault.reload().unwrap();

    msg!(
        "meme_vault_amt {} meme_vault_initial_amt {} quote_vault_amt {} quote_vault_initial_amt {}",
        accs.meme_vault.amount,
        meme_vault_initial_amt,
        accs.quote_vault.amount,
        quote_vault_initial_amt
    );
    let state = &mut accs.staking;

    state.raydium_fees.last_cum_meme_fees = cumulated_fees_meme;
    state.raydium_fees.last_cum_quote_fees = cumulated_fees_quote;

    state.fees_x_total += accs.meme_vault.amount - meme_vault_initial_amt;
    state.fees_y_total += accs.quote_vault.amount - quote_vault_initial_amt;

    Ok(())
}
