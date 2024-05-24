use crate::consts::SLERF_MINT;
use crate::err;
use crate::err::AmmError;
use crate::models::bound::BoundPool;
use crate::models::staked_lp::MemeTicket;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct GetSwapXAmt<'info> {
    pub pool: Account<'info, BoundPool>,
    #[account(constraint = pool.quote_reserve.vault == quote_vault.key())]
    pub quote_vault: Account<'info, TokenAccount>,
}

pub fn handle(ctx: Context<GetSwapXAmt>, coin_in_amount: u64, coin_y_min_value: u64) -> Result<()> {
    let swap_amount = ctx
        .accounts
        .pool
        .swap_amounts(coin_in_amount, coin_y_min_value, false);

    msg!(
        "swapped_in: {}\n swapped_out: {}",
        swap_amount.amount_in,
        swap_amount.amount_out
    );

    Ok(())
}
