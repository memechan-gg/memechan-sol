use crate::consts::ADMIN_KEY;
use crate::models::points_epoch::PointsEpoch;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ChangePointsEpoch<'info> {
    #[account(
        mut,
        constraint = sender.key() == ADMIN_KEY.key()
    )]
    pub sender: Signer<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        space = PointsEpoch::space(),
        seeds = [PointsEpoch::POINTS_EPOCH_PREFIX],
        bump
    )]
    pub points_epoch: Account<'info, PointsEpoch>,
    pub system_program: Program<'info, System>,
}

pub fn handle<'info>(
    ctx: Context<ChangePointsEpoch<'info>>,
    epoch_number: u64,
    points_total: u64,
    points_per_sol_num: u64,
    points_per_sol_denom: u64,
) -> Result<()> {
    let accs = ctx.accounts;

    let epoch = &mut accs.points_epoch;
    epoch.epoch_number = epoch_number;
    epoch.points_total = points_total;
    epoch.points_given = 0;
    epoch.points_per_sol_num = points_per_sol_num;
    epoch.points_per_sol_denom = points_per_sol_denom;

    Ok(())
}
