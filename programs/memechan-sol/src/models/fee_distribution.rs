use crate::err::AmmError;
use crate::{
    math::{Decimal, TryAdd, TryDiv, TryMul, TryRound, TrySub},
    models::staking::StakingPool,
};
use anchor_lang::prelude::*;
use spl_math::uint::U256;

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
    return calc_withdraw_inner(
        fee_state,
        user_stake,
        user_withdrawals_meme,
        user_withdrawals_quote,
    );
}

pub fn calc_withdraw_inner(
    fee_state: &StakingPool,
    user_stake: u64,
    user_withdrawals_meme: u64,
    user_withdrawals_quote: u64,
) -> Result<Withdrawal> {
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
    user_current_stake: u64,
    user_stake_diff: u64,
) -> Result<Withdrawal> {
    let withdrawal = calc_withdraw(state, lp_ticket).unwrap();

    state.stakes_total -= user_stake_diff;

    if state.stakes_total == 0 && user_stake_diff > 0 {
        let withdrawal = Withdrawal {
            max_withdrawal_meme: state.fees_x_total,
            max_withdrawal_quote: state.fees_y_total,
        };

        state.stakes_total = 0;
        state.fees_x_total = 0;
        state.fees_y_total = 0;

        lp_ticket.withdraws_meme = 0;
        lp_ticket.withdraws_quote = 0;

        return Ok(withdrawal);
    }

    let rem_withdrawal =
        calc_withdraw_inner(state, user_current_stake - user_stake_diff, 0, 0).unwrap();

    msg!(
        "lwm {} rwm {} lwq {} rwq {}",
        lp_ticket.withdraws_meme,
        rem_withdrawal.max_withdrawal_meme,
        lp_ticket.withdraws_quote,
        rem_withdrawal.max_withdrawal_quote
    );

    lp_ticket.withdraws_meme = rem_withdrawal.max_withdrawal_meme;
    lp_ticket.withdraws_quote = rem_withdrawal.max_withdrawal_quote;

    state.fees_x_total -= withdrawal.max_withdrawal_meme;
    state.fees_y_total -= withdrawal.max_withdrawal_quote;

    Ok(withdrawal)
}

fn get_max_withdraw(
    user_withdrawals: u64,
    fees_total: u64,
    user_stake: u64,
    stakes_total: u64,
) -> Result<u64> {
    let (user_withdrawals_total, fees_total, user_stake, stakes_total, wad, wad1p) = (
        U256::from(user_withdrawals),
        U256::from(fees_total),
        U256::from(user_stake),
        U256::from(stakes_total),
        U256::from(PRECISION),
        U256::from(PRECISION + PRECISION / 10000),
    );

    let max_user_withdrawal = (fees_total * user_stake * wad) / stakes_total;

    if max_user_withdrawal <= user_withdrawals_total * wad {
        return Err(error!(AmmError::NoTokensToWithdraw));
    }

    let allowed_withdrawal = max_user_withdrawal - user_withdrawals_total * wad;

    Ok((allowed_withdrawal / wad1p).as_u64())
}

fn get_withdraw_diff(user_withdrawals: u64, stake_diff: u128) -> u64 {
    let withdraw_diff_x = ((U256::from(user_withdrawals)) * U256::from(stake_diff)) / PRECISION;
    withdraw_diff_x.as_u64()
}
