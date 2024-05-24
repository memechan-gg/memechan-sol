use crate::err::AmmError;
use crate::{
    math::{Decimal, TryAdd, TryDiv, TryMul, TryRound, TrySub},
    models::staking::StakingPool,
};
use anchor_lang::prelude::*;

use super::staked_lp::MemeTicket;

const PRECISION: u128 = 1_000_000_000_000_000;

pub struct Withdrawal {
    pub max_withdrawal_meme: u64,
    pub max_withdrawal_quote: u64,
}

pub fn calc_withdraw(fee_state: &StakingPool, lp_ticket: &MemeTicket) -> Result<Withdrawal> {
    let user_stake: u64 = lp_ticket.vesting.current_stake();
    let user_withdrawals_meme = lp_ticket.withdraws_meme;
    let user_withdrawals_quote = lp_ticket.withdraws_quote;

    let max_withdrawal_meme = get_max_withdraw(
        user_withdrawals_meme,
        fee_state.fees_x_total,
        user_stake,
        fee_state.stakes_total,
    )
    .unwrap_or(0);

    let max_withdrawal_quote = get_max_withdraw(
        user_withdrawals_quote,
        fee_state.fees_y_total,
        user_stake,
        fee_state.stakes_total,
    )
    .unwrap_or(0);

    Ok(Withdrawal {
        max_withdrawal_meme,
        max_withdrawal_quote,
    })
}

pub fn update_stake(
    state: &mut StakingPool,
    lp_ticket: &mut MemeTicket,
    user_old_stake: u64,
    user_stake_diff: u64,
) -> Result<Withdrawal> {
    let withdrawal = calc_withdraw(state, lp_ticket).unwrap();

    let stake_diff = ((user_stake_diff as u128) * PRECISION) / (user_old_stake as u128);

    let user_withdrawals_x = &mut lp_ticket.withdraws_meme;
    let withdraw_diff_x = get_withdraw_diff(*user_withdrawals_x, stake_diff);
    *user_withdrawals_x -= withdraw_diff_x;

    let user_withdrawals_y = &mut lp_ticket.withdraws_quote;
    let withdraw_diff_y = get_withdraw_diff(*user_withdrawals_y, stake_diff);
    *user_withdrawals_y = withdraw_diff_y;

    state.stakes_total -= user_stake_diff;

    Ok(withdrawal)
}

fn get_max_withdraw(
    user_withdrawals: u64,
    fees_total: u64,
    user_stake: u64,
    stakes_total: u64,
) -> Result<u64> {
    let (user_withdrawals_total, fees_total, user_stake, stakes_total) = (
        user_withdrawals as u128,
        fees_total as u128,
        user_stake as u128,
        stakes_total as u128,
    );

    let max_user_withdrawal = fees_total * ((user_stake * PRECISION) / stakes_total);

    if max_user_withdrawal <= user_withdrawals_total * PRECISION {
        return Err(error!(AmmError::NoTokensToWithdraw));
    }

    let allowed_withdrawal = max_user_withdrawal - user_withdrawals_total;

    Ok((allowed_withdrawal / PRECISION) as u64)
}

fn get_withdraw_diff(user_withdrawals: u64, stake_diff: u128) -> u64 {
    let withdraw_diff_x = ((user_withdrawals as u128) * stake_diff) / PRECISION;
    withdraw_diff_x as u64
}
