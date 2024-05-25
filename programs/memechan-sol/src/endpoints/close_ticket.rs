use crate::err::AmmError;
use crate::models::staked_lp::MemeTicket;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseTicket<'info> {
    #[account(
        mut,
        has_one = owner,
        close = owner
    )]
    pub ticket: Account<'info, MemeTicket>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<CloseTicket>) -> Result<()> {
    if ctx.accounts.ticket.amount != 0 {
        return Err(error!(AmmError::NonZeroAmountTicket));
    }

    Ok(())
}
