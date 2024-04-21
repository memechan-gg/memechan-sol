use crate::err::AmmError;
use anchor_lang::prelude::*;

const PRECISION: u128 = 1_000_000_000_000_000_000;

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Fees {
    pub fee_in_percent: u128,
    pub fee_out_percent: u128,
}

pub fn get_fee_in_amount(fees: &Fees, amount: u64) -> Result<u64> {
    get_fee_amount(amount, fees.fee_in_percent)
}

pub fn get_fee_out_amount(fees: &Fees, amount: u64) -> Result<u64> {
    get_fee_amount(amount, fees.fee_out_percent)
}

pub fn get_fee_in_initial_amount(fees: &Fees, amount: u64) -> Result<u64> {
    get_initial_amount(amount, fees.fee_in_percent)
}

pub fn get_fee_out_initial_amount(fees: &Fees, amount: u64) -> Result<u64> {
    get_initial_amount(amount, fees.fee_out_percent)
}

fn get_fee_amount(x: u64, percent: u128) -> Result<u64> {
    Ok(checked_mul_div_round_up(x as u128, percent, PRECISION).unwrap() as u64)
}

fn get_initial_amount(x: u64, percent: u128) -> Result<u64> {
    Ok(checked_mul_div_round_up(x as u128, PRECISION, PRECISION - percent).unwrap() as u64)
}

//Orca's whirlpools math
#[allow(dead_code)]
pub fn checked_mul_div(n0: u128, n1: u128, d: u128) -> Result<u128> {
    checked_mul_div_round_up_if(n0, n1, d, false)
}

pub fn checked_mul_div_round_up(n0: u128, n1: u128, d: u128) -> Result<u128> {
    checked_mul_div_round_up_if(n0, n1, d, true)
}

pub fn checked_mul_div_round_up_if(n0: u128, n1: u128, d: u128, round_up: bool) -> Result<u128> {
    if d == 0 {
        return Err(error!(AmmError::DivideByZero));
    }

    let p = n0.checked_mul(n1).ok_or(error!(AmmError::MulDivOverflow))?;
    let n = p / d;

    Ok(if round_up && p % d > 0 { n + 1 } else { n })
}
