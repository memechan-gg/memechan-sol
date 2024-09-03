use crate::{
    consts::RAYDIUM_PROGRAM_ID,
    models::staking::{lp_tokens_to_burn, StakingPool},
    raydium::{self, models::AmmInfo},
};

use crate::models::OpenBook;
use crate::raydium::RaydiumAmm;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct AddFees<'info> {
    #[account(
        mut,
        has_one = meme_vault,
        has_one = quote_vault,
        has_one = raydium_amm
    )]
    pub staking: Account<'info, StakingPool>,
    #[account(mut)]
    pub meme_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub quote_vault: Account<'info, TokenAccount>,
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
    pub raydium_amm: AccountInfo<'info>,
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
    // Open Book
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub target_orders: AccountInfo<'info>,
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub market_account: AccountInfo<'info>,
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub market_event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub market_coin_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub market_pc_vault: Box<Account<'info, TokenAccount>>,
    /// CHECK: Checks done in cpi call to raydium
    pub market_vault_signer: AccountInfo<'info>,
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub market_bids: AccountInfo<'info>,
    /// CHECK: Checks done in cpi call to raydium
    #[account(mut)]
    pub market_asks: AccountInfo<'info>,
    // Programs
    pub token_program: Program<'info, Token>,
    pub raydium_program: Program<'info, RaydiumAmm>,
    pub market_program_id: Program<'info, OpenBook>,
}

impl<'info> AddFees<'info> {
    pub fn redeem_liquidity(&self, amount: u64, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let instruction = raydium::withdraw(
            &self.raydium_program.key(),
            // params
            amount,
            // accounts
            &self.token_program.key(),
            &self.raydium_amm.key(),
            &self.raydium_amm_authority.key(),
            &self.open_orders.key(),
            &self.target_orders.key(),
            &self.raydium_lp_mint.key(),     // lp mint
            &self.raydium_meme_vault.key(),  // coin_vault
            &self.raydium_quote_vault.key(), // pc_vault
            &self.market_program_id.key(),
            &self.market_account.key(),
            &self.market_coin_vault.key(),
            &self.market_pc_vault.key(),
            &self.market_vault_signer.key(),
            &self.staking_lp_wallet.key(),
            &self.meme_vault.key(),         // user wallet (pool)
            &self.quote_vault.key(),        // user wallet (pool)
            &self.staking_signer_pda.key(), // user wallet
            &self.market_event_queue.key(),
            &self.market_bids.key(),
            &self.market_asks.key(),
        );

        solana_program::program::invoke_signed(
            &instruction,
            &[
                self.token_program.to_account_info().clone(),
                self.raydium_amm.to_account_info().clone(),
                self.raydium_amm_authority.to_account_info().clone(),
                self.open_orders.to_account_info().clone(),
                self.target_orders.to_account_info().clone(),
                self.raydium_lp_mint.to_account_info().clone(),
                self.raydium_meme_vault.to_account_info().clone(),
                self.raydium_quote_vault.to_account_info().clone(),
                self.market_program_id.to_account_info().clone(),
                self.market_account.to_account_info().clone(),
                self.market_coin_vault.to_account_info().clone(),
                self.market_pc_vault.to_account_info().clone(),
                self.market_vault_signer.to_account_info().clone(),
                self.staking_lp_wallet.to_account_info().clone(),
                self.meme_vault.to_account_info().clone(),
                self.quote_vault.to_account_info().clone(),
                self.staking_signer_pda.to_account_info().clone(),
                self.market_event_queue.to_account_info().clone(),
                self.market_bids.to_account_info().clone(),
                self.market_asks.to_account_info().clone(),
            ],
            signer_seeds,
        )?;

        Ok(())
    }

    // pub fn withdraw_fees_ctx(&self) -> CpiContext<'_, '_, '_, 'info, RedeemLiquidity<'info>> {
    //     let cpi_program = self.aldrin_amm_program.to_account_info();
    //     let cpi_accounts = RedeemLiquidity {
    //         user: self.staking_signer_pda.to_account_info(),
    //         pool: self.aldrin_pool_acc.to_account_info(),
    //         pool_signer: self.aldrin_pool_signer.to_account_info(),
    //         lp_mint: self.aldrin_lp_mint.to_account_info(),
    //         lp_token_wallet: self.aldrin_pool_lp_wallet.to_account_info(),
    //         token_program: self.token_program.to_account_info(),
    //     };
    //     CpiContext::new(cpi_program, cpi_accounts)
    // }
}

pub fn handle<'info>(ctx: Context<'_, '_, '_, 'info, AddFees<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    if accs.staking_lp_wallet.amount == 0 {
        return Ok(());
    }

    let meme_vault_initial_amt = accs.meme_vault.amount;
    let quote_vault_initial_amt = accs.quote_vault.amount;

    let amm_info = &accs.raydium_amm.clone();
    let amm = AmmInfo::load_checked(amm_info, &RAYDIUM_PROGRAM_ID).unwrap();

    let cumulated_fees_meme = amm.state_data.swap_acc_coin_fee;
    let cumulated_fees_quote = amm.state_data.swap_acc_pc_fee;

    drop(amm);

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

    let state = &mut accs.staking;

    state.raydium_fees.last_cum_meme_fees = cumulated_fees_meme;
    state.raydium_fees.last_cum_quote_fees = cumulated_fees_quote;

    state.fees_x_total += accs.meme_vault.amount - meme_vault_initial_amt;
    state.fees_y_total += accs.quote_vault.amount - quote_vault_initial_amt;

    Ok(())
}
