use crate::consts::ADMIN_KEY;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::models::staked_lp::MemeTicket;
use crate::models::ticket_schedule::TicketSchedule;

#[derive(Accounts)]
pub struct NewTicketSchedule<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        init,
        payer = sender,
        space = TicketSchedule::space(),
        seeds = [TicketSchedule::SCHEDULE_PREFIX, meme_ticket.key().as_ref()],
        bump
    )]
    pub ticket_schedule: Account<'info, TicketSchedule>,
    pub meme_ticket: Account<'info, MemeTicket>,
    pub system_program: Program<'info, System>,
}

pub fn handle<'info>(ctx: Context<NewTicketSchedule<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    let schedule = &mut accs.ticket_schedule;
    schedule.meme_ticket = accs.meme_ticket.key();
    schedule.withdrawn = 0;
    schedule.until_ts = 0;

    Ok(())
}
