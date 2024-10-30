use crate::models::user_points::UserPoints;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct NewUserPoints<'info> {
    #[account(
       init,
       payer = owner,
       space = UserPoints::space(),
       seeds = [UserPoints::USER_POINTS_PREFIX, owner.key().as_ref()],
       bump
    )]
    pub user_points: Account<'info, UserPoints>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: doesn't need validation
    #[account(constraint = referrer.key() != owner.key())]
    pub referrer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<NewUserPoints>) -> Result<()> {
    ctx.accounts.user_points.referrer = ctx.accounts.referrer.key();

    Ok(())
}
