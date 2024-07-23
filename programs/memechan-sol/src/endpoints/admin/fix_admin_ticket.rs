use crate::consts::{ADMIN_KEY, DEFAULT_MAX_M};
use crate::models::fee_distribution::calc_withdraw;
use crate::models::fees::get_admin_fee_position_size;
use crate::models::meme_ticket::MemeTicket;
use crate::models::staking::StakingPool;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct FixAdminTicket<'info> {
    #[account(
        mut,
        constraint = sender.key() == ADMIN_KEY.key()
    )]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub staking: Box<Account<'info, StakingPool>>,
    #[account(
        mut,
        constraint = admin_ticket.pool == staking.pool,
        constraint = admin_ticket.vesting.notional == 0,
        seeds = [MemeTicket::ADMIN_TICKET_PREFIX, staking.key().as_ref()],
        bump
    )]
    pub admin_ticket: Box<Account<'info, MemeTicket>>,
}

pub fn handle<'info>(ctx: Context<FixAdminTicket<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    let ticket_amt = get_admin_fee_position_size(DEFAULT_MAX_M)?;

    let admin_ticket = &mut accs.admin_ticket;
    admin_ticket.vesting.notional = ticket_amt;

    let staking = &mut accs.staking;
    staking.stakes_total += ticket_amt;
    staking.admin_fee_position = ticket_amt;

    let withdrawal = calc_withdraw(staking, admin_ticket).unwrap();

    admin_ticket.withdraws_meme = withdrawal.max_withdrawal_meme;
    admin_ticket.withdraws_quote = withdrawal.max_withdrawal_quote;
    admin_ticket.withdraws_chan = withdrawal.max_withdrawal_chan;

    Ok(())
}
