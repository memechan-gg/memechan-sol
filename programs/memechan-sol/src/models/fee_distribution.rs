use crate::err::AmmError;
use crate::models::staking::StakingPool;
use anchor_lang::prelude::*;
use spl_math::uint::U256;
use std::ops::{Div, Mul};

use super::staked_lp::MemeTicket;

const PRECISION: u128 = 1_000_000_000_000_000;

#[derive(Debug)]
pub struct Withdrawal {
    pub max_withdrawal_meme: u64,
    pub max_withdrawal_quote: u64,
    pub max_withdrawal_chan: u64,
}

pub fn calc_withdraw(fee_state: &StakingPool, lp_ticket: &MemeTicket) -> Result<Withdrawal> {
    let user_stake: u64 = lp_ticket.vesting.current_stake();
    let user_withdrawals_meme = lp_ticket.withdraws_meme;
    let user_withdrawals_quote = lp_ticket.withdraws_quote;
    let user_withdrawals_chan = lp_ticket.withdraws_chan;

    return calc_withdraw_inner(
        fee_state,
        user_stake,
        user_withdrawals_meme,
        user_withdrawals_quote,
        user_withdrawals_chan,
    );
}

pub fn calc_withdraw_inner(
    fee_state: &StakingPool,
    user_stake: u64,
    user_withdrawals_meme: u64,
    user_withdrawals_quote: u64,
    user_withdrawals_chan: u64,
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

    let max_withdrawal_chan = get_max_withdraw(
        user_withdrawals_chan,
        fee_state.fees_z_total,
        user_stake,
        fee_state.stakes_total,
    )
    .unwrap_or(0);

    Ok(Withdrawal {
        max_withdrawal_meme,
        max_withdrawal_quote,
        max_withdrawal_chan,
    })
}

pub fn update_stake(
    state: &mut StakingPool,
    lp_ticket: &mut MemeTicket,
    user_old_stake: u64,
    user_stake_diff: u64,
) -> Result<Withdrawal> {
    let withdrawal = calc_withdraw(state, lp_ticket).unwrap();

    let new_stakes_total = state.stakes_total.checked_sub(user_stake_diff).unwrap();

    if new_stakes_total == 0 && user_stake_diff > 0 {
        let withdrawal = Withdrawal {
            max_withdrawal_meme: state.fees_x_total,
            max_withdrawal_quote: state.fees_y_total,
            max_withdrawal_chan: state.fees_z_total,
        };

        state.stakes_total = 0;
        state.fees_x_total = 0;
        state.fees_y_total = 0;
        state.fees_z_total = 0;

        lp_ticket.withdraws_meme = 0;
        lp_ticket.withdraws_quote = 0;
        lp_ticket.withdraws_chan = 0;

        return Ok(withdrawal);
    }

    state.fees_x_total = mul_div(state.fees_x_total, new_stakes_total, state.stakes_total).unwrap();
    state.fees_y_total = mul_div(state.fees_y_total, new_stakes_total, state.stakes_total).unwrap();
    state.fees_z_total = mul_div(state.fees_z_total, new_stakes_total, state.stakes_total).unwrap();
    state.stakes_total = new_stakes_total;

    msg!(
        "lwm {} lwq {} lwc {}",
        lp_ticket.withdraws_meme,
        lp_ticket.withdraws_quote,
        lp_ticket.withdraws_chan,
    );

    let user_new_stake = user_old_stake.checked_sub(user_stake_diff).unwrap();
    lp_ticket.withdraws_meme = mul_div(
        lp_ticket.withdraws_meme + withdrawal.max_withdrawal_meme,
        user_new_stake,
        user_old_stake,
    )
    .unwrap();
    lp_ticket.withdraws_quote = mul_div(
        lp_ticket.withdraws_quote + withdrawal.max_withdrawal_quote,
        user_new_stake,
        user_old_stake,
    )
    .unwrap();
    lp_ticket.withdraws_chan = mul_div(
        lp_ticket.withdraws_chan + withdrawal.max_withdrawal_chan,
        user_new_stake,
        user_old_stake,
    )
    .unwrap();

    Ok(withdrawal)
}

fn mul_div(amt: u64, num: u64, denom: u64) -> Result<u64> {
    let (amt, num, denom) = (U256::from(amt), U256::from(num), U256::from(denom));

    Ok(amt.mul(num).div(denom).as_u64())
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
