use crate::models::bound::BoundPool;
use crate::models::staked_lp::MemeTicket;
use anchor_lang::prelude::*;
use std::cmp::max;

#[derive(Accounts)]
pub struct BoundMergeTickets<'info> {
    pub pool: Account<'info, BoundPool>,
    #[account(
        mut,
        has_one = owner,
        constraint = ticket_into.pool == ticket_from.pool,
        constraint = ticket_into.pool == pool.key(),
        constraint = ticket_into.key() != ticket_from.key()
    )]
    pub ticket_into: Account<'info, MemeTicket>,
    #[account(
        mut,
        close = owner,
        has_one = owner
    )]
    pub ticket_from: Account<'info, MemeTicket>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handle(ctx: Context<BoundMergeTickets>) -> Result<()> {
    let accs = ctx.accounts;
    let ticket_into = &mut accs.ticket_into;
    let ticket_from = &mut accs.ticket_from;

    ticket_into.amount += ticket_from.amount;
    ticket_into.withdraws_wsol += ticket_from.withdraws_wsol;
    ticket_into.withdraws_meme += ticket_from.withdraws_meme;
    ticket_into.vesting.notional += ticket_from.vesting.notional;
    ticket_into.vesting.released += ticket_from.vesting.released;
    ticket_into.until_timestamp = max(ticket_into.until_timestamp, ticket_from.until_timestamp);

    Ok(())
}
