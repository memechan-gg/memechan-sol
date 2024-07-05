use crate::consts::SWAP_AUTH_KEY;
use crate::models::chan_swap::ChanSwap;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ChangeChanPrice<'info> {
    #[account(constraint = sender.key() == SWAP_AUTH_KEY.key())]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub chan_swap: Account<'info, ChanSwap>,
}

pub fn handle<'info>(
    ctx: Context<ChangeChanPrice<'info>>,
    new_price_num: u64,
    new_price_denom: u64,
) -> Result<()> {
    let chan_swap = &mut ctx.accounts.chan_swap;

    chan_swap.chan_sol_price_num = new_price_num;
    chan_swap.chan_sol_price_denom = new_price_denom;

    Ok(())
}
