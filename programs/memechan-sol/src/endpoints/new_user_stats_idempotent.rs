use anchor_lang::prelude::*;
use anchor_lang::Accounts;

use crate::models::user_stats::UserStats;

#[derive(Accounts)]
pub struct NewUserStatsIdempotent<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    ///CHECK: doesn't need one
    pub referral: AccountInfo<'info>,
    ///CHECK: doesn't need one
    pub pool: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        space = UserStats::space(),
        seeds = [UserStats::STATS_PREFIX, referral.key().as_ref(), pool.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<NewUserStatsIdempotent>) -> Result<()> {
    let accs = ctx.accounts;
    let user_stats = &mut accs.user_stats;

    if user_stats.is_initialized {
        return Ok(());
    }

    user_stats.is_initialized = true;

    user_stats.referral = accs.referral.key();
    user_stats.pool = accs.pool.key();

    Ok(())
}
