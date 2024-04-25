const PRECISION: u128 = 1_000_000_000_000_000_000;

const MAX_X: u128 = 900_000_000 * PRECISION;
const MAX_Y: u128 = 30_000 * PRECISION;

const DECIMALS_X: u128 = 1_000_000;
const DECIMALS_Y: u128 = 1_000_000_000;

const PAD_DECIMALS: u128 = 100;

use crate::err::AmmError;
use anchor_lang::error;
use anchor_lang::prelude::*;
use num_integer::*;

pub fn invariant(x: u64, y: u64) -> Result<u128> {
    let res_y = MAX_Y - (y as u128);
    Ok((x as u128) - res_y * res_y)
}

pub fn get_amount_out(
    coin_in_amount: u64,
    balance_x: u64,
    balance_y: u64,
    is_x: bool,
) -> Result<u64> {
    if coin_in_amount == 0 {
        return Err(error!(AmmError::ZeroInAmt));
    }
    if balance_x == 0 {
        return Err(error!(AmmError::ZeroMemeVault));
    }
    let is_balance_sufficient = if is_x {
        balance_x.checked_sub(coin_in_amount).is_some()
    } else {
        balance_y >= coin_in_amount
    };
    if !is_balance_sufficient {
        return Err(error!(AmmError::InsufficientBalance));
    }

    let (coin_in_amount, balance_x, balance_y) = (
        ((coin_in_amount as u128) * PRECISION)
            / if is_x {
                DECIMALS_X
            } else {
                DECIMALS_Y / PAD_DECIMALS
            },
        (balance_x as u128 * PRECISION) / DECIMALS_X,
        ((balance_y as u128 * PAD_DECIMALS) * PRECISION) / DECIMALS_Y,
    );

    let res_y = MAX_Y - balance_y;
    let res_x = MAX_X - balance_x;

    let res = if is_x {
        let new_balance_x = res_x + coin_in_amount;

        u128::sqrt(&new_balance_x) - &res_x.sqrt()
    } else {
        let new_balance_y = res_y - coin_in_amount;

        res_y * res_y - new_balance_y * new_balance_y
    };

    let nres =
        (res * if is_x {
            DECIMALS_Y / PAD_DECIMALS
        } else {
            DECIMALS_X
        }) / (PRECISION * PRECISION);

    Ok(nres as u64)
}
