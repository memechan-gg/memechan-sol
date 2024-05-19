use crate::{
    consts::{DECIMALS_ALPHA, DECIMALS_BETA, DECIMALS_S},
    err::AmmError,
};
use anchor_lang::prelude::*;
use num_integer::Roots;
use solana_program::pubkey::Pubkey;
use std::{cmp::min, mem};

use super::{fees::Fees, mist, Reserve, SwapAmount};

#[account]
pub struct BoundPool {
    pub meme_reserve: Reserve,
    pub quote_reserve: Reserve,
    pub admin_fees_meme: u64,
    pub admin_fees_quote: u64,
    pub admin_vault_quote: Pubkey,
    pub fees: Fees,
    pub config: Config,
    pub locked: bool,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Config {
    pub alpha_abs: u128, // |alpha|, because alpha is negative
    pub beta: u128,
    pub price_factor: u64,
    // In quote denomination
    pub gamma_s: u64,
    // In raw denomination
    pub gamma_m: u64, // DEFAULT_MAX_M * DECIMALS_M = 900_000_000_000_000
    // In raw denomination
    pub omega_m: u64, // DEFAULT_MAX_M_LP * DECIMALS_M = 200_000_000_000_000
}

impl BoundPool {
    pub fn swap_amounts(
        &self,
        coin_in_amount: u64,
        coin_out_min_value: u64,
        buy_meme: bool,
    ) -> SwapAmount {
        if buy_meme {
            self.buy_meme_swap_amounts(coin_in_amount, coin_out_min_value)
                .unwrap()
        } else {
            self.sell_meme_swap_amounts(coin_in_amount, coin_out_min_value)
                .unwrap()
        }
    }

    fn buy_meme_swap_amounts(&self, delta_s: u64, min_delta_m: u64) -> Result<SwapAmount> {
        let (m_t0, s_t0) = self.balances();

        let p = &self.config;

        let max_delta_s = (p.gamma_s_mist()) - s_t0;

        let admin_fee_in = self.fees.get_fee_in_amount(delta_s).unwrap();
        let is_max = delta_s - admin_fee_in >= max_delta_s;

        let net_delta_s = min(delta_s - admin_fee_in, max_delta_s);

        let delta_m = if is_max {
            m_t0
        } else {
            self.compute_delta_m(s_t0, s_t0 + net_delta_s)
        };

        let admin_fee_out = self.fees.get_fee_out_amount(delta_m).unwrap();
        let net_delta_m = delta_m - admin_fee_out;

        //assert!(net_delta_m >= min_delta_m, errors::slippage());
        if net_delta_m < min_delta_m {
            return Err(error!(AmmError::SlippageExceeded));
        }

        Ok(SwapAmount {
            amount_in: net_delta_s,
            amount_out: net_delta_m,
            admin_fee_in,
            admin_fee_out,
        })
    }

    fn sell_meme_swap_amounts(&self, delta_m: u64, min_delta_s: u64) -> Result<SwapAmount> {
        let (m_b, s_b) = self.balances();

        let p = &self.config;

        let max_delta_m = p.gamma_m - m_b; // TODO: confirm

        let admin_fee_in = self.fees.get_fee_in_amount(delta_m).unwrap();
        let is_max = delta_m - admin_fee_in > max_delta_m; // TODO: shouldn't it be >=?

        let net_delta_m = min(delta_m - admin_fee_in, max_delta_m);

        let delta_s = if is_max {
            s_b // TODO: confirm
        } else {
            self.compute_delta_s(s_b, net_delta_m)
        };

        let admin_fee_out = self.fees.get_fee_out_amount(delta_s).unwrap();
        let net_delta_s = delta_s - admin_fee_out;

        //assert!(net_delta_s >= min_delta_s, errors::slippage());
        if net_delta_s < min_delta_s {
            return Err(error!(AmmError::SlippageExceeded));
        }

        Ok(SwapAmount {
            amount_in: net_delta_m,
            amount_out: net_delta_s,
            admin_fee_in,
            admin_fee_out,
        })
    }

    pub fn compute_delta_m(&self, s_a: u64, s_b: u64) -> u64 {
        let s_a = s_a as u128;
        let s_b = s_b as u128;

        let alpha_abs = &self.config.alpha_abs;
        let beta = &self.config.beta;

        let left = *beta * (s_b - s_a) / (DECIMALS_BETA * DECIMALS_S);
        let pow_decimals = DECIMALS_S * DECIMALS_S;
        let right = *alpha_abs * ((s_b * s_b) / pow_decimals - (s_a * s_a) / pow_decimals)
            / (2 * DECIMALS_ALPHA);

        (left - right) as u64
    }

    pub fn compute_delta_s(&self, s_b: u64, delta_m: u64) -> u64 {
        let s_b = s_b as u128;
        let delta_m = delta_m as u128;

        let alpha_abs = self.config.alpha_abs;
        let beta = self.config.beta;

        let b_hat_abs = ((2 * beta * DECIMALS_ALPHA * DECIMALS_S)
            - (2 * alpha_abs * s_b * DECIMALS_BETA))
            / (DECIMALS_ALPHA * DECIMALS_BETA * DECIMALS_S);

        // SQRT
        let sqrt_term = ((((b_hat_abs * b_hat_abs) * DECIMALS_ALPHA) + (8 * delta_m * alpha_abs))
            / DECIMALS_ALPHA)
            .sqrt();

        let num = sqrt_term - b_hat_abs;

        ((num * DECIMALS_ALPHA * DECIMALS_S) / (2 * alpha_abs)) as u64
    }

    fn balances(&self) -> (u64, u64) {
        (self.meme_reserve.tokens, self.quote_reserve.tokens)
    }
}

impl Config {
    fn gamma_s_mist(&self) -> u64 {
        mist(self.gamma_s)
    }
}

pub fn compute_alpha_abs(
    gamma_s: u128,
    gamma_m: u128,
    omega_m: u128,
    price_factor: u64,
) -> Result<u128> {
    let left = omega_m * (price_factor as u128);
    //assert!(left < gamma_m, EBondingCurveMustBeNegativelySloped);
    if left >= gamma_m {
        return Err(error!(AmmError::BondingCurveMustBeNegativelySloped));
    }

    // We compute |alpha|, hence the subtraction is switched
    Ok((2 * (gamma_m - left) * DECIMALS_ALPHA) / (gamma_s * gamma_s))
}

pub fn compute_beta(
    gamma_s: u128,
    gamma_m: u128,
    omega_m: u128,
    price_factor: u64,
) -> Result<u128> {
    let left = 2 * gamma_m;
    let right = omega_m * (price_factor as u128);
    //assert!(left > right, EBondingCurveInterceptMustBePositive);
    if left <= gamma_m {
        return Err(error!(AmmError::BondingCurveInterceptMustBePositive));
    }

    Ok(((left - right) * DECIMALS_BETA) / gamma_s)
}

impl BoundPool {
    pub const POOL_PREFIX: &'static [u8; 10] = b"bound_pool";
    pub const SIGNER_PDA_PREFIX: &'static [u8; 6] = b"signer";

    pub fn space() -> usize {
        let discriminant = 8;

        let meme_reserve = mem::size_of::<Reserve>();
        let quote_reserve = mem::size_of::<Reserve>();
        let admin_fees_meme = 8;
        let admin_fees_quote = 8;
        let admin_vault_quote = 32;
        let fees = mem::size_of::<Fees>();
        let config = mem::size_of::<Config>();
        let locked = 1;

        discriminant
            + meme_reserve
            + quote_reserve
            + admin_fees_meme
            + admin_fees_quote
            + admin_vault_quote
            + fees
            + config
            + locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DECIMALS_ALPHA: u128 = 100; // Example value for DECIMALS_ALPHA

    #[test]
    fn test_compute_alpha_abs_valid() {
        // Test when left < gamma_m
        let gamma_s = 10;
        let gamma_m = 20;
        let omega_m = 30;
        let price_factor = 1;

        let result = compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor);
        assert!(result.is_ok());

        // You can add more test cases here for different valid inputs
    }

    #[test]
    fn test_compute_alpha_abs_invalid() {
        // Test when left >= gamma_m
        let gamma_s = 10;
        let gamma_m = 20;
        let omega_m = 30;
        let price_factor = 1;

        let result = compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor);
        assert!(result.is_err());

        // You can add more test cases here for different invalid inputs
    }
}
