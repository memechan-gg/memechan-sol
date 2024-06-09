use std::cmp::max;
use crate::models::staked_lp::MemeTicket;
use crate::models::staking::StakingPool;
use anchor_lang::prelude::*;
use crate::models::ticket_schedule::TicketSchedule;

#[derive(Accounts)]
pub struct StakingMergeTickets<'info> {
    pub staking: Account<'info, StakingPool>,
    #[account(
        mut,
        has_one = owner,
        constraint = ticket_into.pool == ticket_from.pool,
        constraint = ticket_into.pool == staking.pool,
        constraint = ticket_into.key() != ticket_from.key()
    )]
    pub ticket_into: Account<'info, MemeTicket>,
    #[account(
        mut,
        close = owner,
        has_one = owner
    )]
    pub ticket_from: Account<'info, MemeTicket>,
    #[account(
        mut,
        constraint = ticket_into.key() == schedule_into.meme_ticket
    )]
    pub schedule_into : Account<'info, TicketSchedule>,
    #[account(
        mut,
        close = owner,
        constraint = ticket_from.key() == schedule_from.meme_ticket
    )]
    pub schedule_from : Account<'info, TicketSchedule>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handle(ctx: Context<StakingMergeTickets>) -> Result<()> {
    let accs = ctx.accounts;
    let ticket_into = &mut accs.ticket_into;
    let ticket_from = &mut accs.ticket_from;

    ticket_into.amount += ticket_from.amount;
    ticket_into.withdraws_quote += ticket_from.withdraws_quote;
    ticket_into.withdraws_meme += ticket_from.withdraws_meme;
    ticket_into.vesting.notional += ticket_from.vesting.notional;
    ticket_into.vesting.released += ticket_from.vesting.released;

    let schedule_into = &mut accs.schedule_into;
    let schedule_from = &mut accs.schedule_from;

    schedule_into.withdrawn += schedule_from.withdrawn;
    schedule_into.until_ts = max(schedule_into.until_ts, schedule_from.until_ts);

    Ok(())
}
