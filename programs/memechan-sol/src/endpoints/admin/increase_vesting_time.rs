use crate::consts::ADMIN_KEY;
use anchor_lang::prelude::*;
use crate::models::staking::StakingPool;

#[derive(Accounts)]
pub struct IncreaseVestingTime<'info> {
    #[account(
        constraint = sender.key() == ADMIN_KEY.key()
    )]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub staking: Account<'info, StakingPool>,
}

pub fn handle<'info>(ctx: Context<IncreaseVestingTime<'info>>, vesting_ts_increase: u64) -> Result<()> {
    let staking = &mut ctx.accounts.staking;

    staking.vesting_config.end_ts += vesting_ts_increase as i64;

    Ok(())
}
