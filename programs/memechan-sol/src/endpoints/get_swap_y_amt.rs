use crate::models::bound::BoundPool;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct GetSwapYAmt<'info> {
    pub pool: Account<'info, BoundPool>,
    #[account(constraint = pool.quote_reserve.vault == quote_vault.key())]
    pub quote_vault: Account<'info, TokenAccount>,
}

pub fn handle(ctx: Context<GetSwapYAmt>, coin_in_amount: u64, coin_x_min_value: u64) -> Result<()> {
    let swap_amount = ctx
        .accounts
        .pool
        .swap_amounts(coin_in_amount, coin_x_min_value, true);

    msg!(
        "swapped_in: {}\n swapped_out: {}",
        swap_amount.amount_in,
        swap_amount.amount_out
    );

    Ok(())
}
