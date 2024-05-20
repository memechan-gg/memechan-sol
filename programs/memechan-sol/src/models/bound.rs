use crate::{consts::DECIMALS_S, err::AmmError};
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
pub struct Decimals {
    pub alpha: u128,
    pub beta: u128,
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
    pub decimals: Decimals,
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

        let decimals_alpha = self.config.decimals.alpha;
        let decimals_beta = self.config.decimals.beta;

        let alpha_abs = &self.config.alpha_abs;
        let beta = &self.config.beta;

        let left = *beta * (s_b - s_a) / (decimals_beta * DECIMALS_S);
        let pow_decimals = DECIMALS_S * DECIMALS_S;
        let right = *alpha_abs * ((s_b * s_b) / pow_decimals - (s_a * s_a) / pow_decimals)
            / (2 * decimals_alpha);

        (left - right) as u64
    }

    pub fn compute_delta_s(&self, s_b: u64, delta_m: u64) -> u64 {
        let decimals_alpha = self.config.decimals.alpha;
        let decimals_beta = self.config.decimals.beta;

        let s_b = s_b as u128;
        let delta_m = delta_m as u128;

        let alpha_abs = self.config.alpha_abs;
        let beta = self.config.beta;

        let b_hat_abs = ((2 * beta * decimals_alpha * DECIMALS_S)
            - (2 * alpha_abs * s_b * decimals_beta))
            / (decimals_alpha * decimals_beta * DECIMALS_S);

        // SQRT
        let sqrt_term = ((((b_hat_abs * b_hat_abs) * decimals_alpha) + (8 * delta_m * alpha_abs))
            / decimals_alpha)
            .sqrt();

        let num = sqrt_term - b_hat_abs;

        ((num * decimals_alpha * DECIMALS_S) / (2 * alpha_abs)) as u64
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
) -> Result<(u128, u128)> {
    check_slope(gamma_m, omega_m, price_factor)?;

    let left = omega_m * (price_factor as u128);

    let num = 2 * (gamma_m - left);
    let denom = gamma_s * gamma_s;

    if num <= denom {
        return Err(error!(AmmError::EGammaSAboveRelativeLimit));
    }

    let num_scale = compute_scale(num);
    let denom_scale = compute_scale(denom);

    let net_scale = num_scale - denom_scale;

    let alpha_decimals = compute_decimals(net_scale)?;

    // We compute |alpha|, hence the subtraction is switched
    Ok(((num * alpha_decimals) / denom, alpha_decimals))
}

pub fn compute_alpha_abs_with_decimals(
    gamma_s: u128,
    gamma_m: u128,
    omega_m: u128,
    price_factor: u64,
    decimals: u128,
) -> Result<u128> {
    check_slope(gamma_m, omega_m, price_factor)?;

    let left = omega_m * (price_factor as u128);

    let num = 2 * (gamma_m - left);
    let denom = gamma_s * gamma_s;

    if num <= denom {
        return Err(error!(AmmError::EGammaSAboveRelativeLimit));
    }

    // We compute |alpha|, hence the subtraction is switched
    Ok((num * decimals) / denom)
}

pub fn compute_decimals(scale: u64) -> Result<u128> {
    match scale {
        0..=4 => return Err(error!(AmmError::EScaleTooLow)),
        5 => Ok(100_000_000),
        6 => Ok(10_000_000),
        7 => Ok(1_000_000),
        8 => Ok(100_000),
        9 => Ok(10_000),
        10 => Ok(1_000),
        11 => Ok(100),
        12 => Ok(10),
        _ => Ok(1), // If scale is above 12
    }
}

pub fn compute_beta(
    gamma_s: u128,
    gamma_m: u128,
    omega_m: u128,
    price_factor: u64,
    beta_decimals: u128,
) -> Result<u128> {
    check_intercept(gamma_m, omega_m, price_factor)?;

    let left = 2 * gamma_m;
    let right = omega_m * (price_factor as u128);

    let num = left - right;
    let denom = gamma_s;

    Ok((num * beta_decimals) / denom)
}

pub fn check_slope(gamma_m: u128, omega_m: u128, price_factor: u64) -> Result<()> {
    if price_factor as u128 * omega_m >= gamma_m {
        return Err(error!(AmmError::BondingCurveMustBeNegativelySloped));
    }

    Ok(())
}

pub fn check_intercept(gamma_m: u128, omega_m: u128, price_factor: u64) -> Result<()> {
    if 2 * gamma_m <= omega_m * (price_factor as u128) {
        return Err(error!(AmmError::BondingCurveInterceptMustBePositive));
    }

    Ok(())
}

fn compute_scale(num_: u128) -> u64 {
    let mut num = num_;

    if num == 0 {
        return 1;
    } else {
        let mut scale = 1;

        while num >= 10 {
            num = num / 10;
            scale += 1;
        }

        return scale;
    }
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

    pub const DEFAULT_DECIMALS_ALPHA: u128 = 1_000_000;
    pub const DEFAULT_DECIMALS_BETA: u128 = 1_000_000;

    #[test]
    fn test_compute_alpha_abs_valid() {
        // Test when left < gamma_m
        let gamma_s = 1;
        let gamma_m = 300_000;
        let omega_m = 200_000;
        let price_factor = 1;

        let result = compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compute_alpha_abs_invalid() {
        // === Scale Too Low ===

        let gamma_s = 1;
        let gamma_m = 30_000;
        let omega_m = 20_000;
        let price_factor = 1;

        let result = compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor);
        assert!(result.is_err());

        // === Positive Slope ===

        let gamma_s = 1;
        let gamma_m = 300_000;
        let omega_m = 200_000;
        let price_factor = 20;
        let result = compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor);
        assert!(result.is_err());

        let result = compute_beta(
            gamma_s,
            gamma_m,
            omega_m,
            price_factor,
            DEFAULT_DECIMALS_BETA,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_alpha_abs_with_pf_2() -> Result<()> {
        let gamma_s = vec![
            30000,  //
            40000,  //
            50000,  //
            60000,  //
            70000,  //
            80000,  //
            90000,  //
            100000, //
            110000, //
            120000, //
            130000, //
            140000, //
            150000, //
            160000, //
            170000, //
            180000, //
            190000, //
            200000, //
            210000, //
            220000, //
            230000, //
        ];

        // Test when left < gamma_m
        let gamma_m = 900000000000000;
        let omega_m = 200000000000000;
        let price_factor = 2;

        let actual_alpha = gamma_s
            .iter()
            .map(|g_s| {
                compute_alpha_abs_with_decimals(
                    *g_s as u128,
                    gamma_m,
                    omega_m,
                    price_factor,
                    DEFAULT_DECIMALS_ALPHA,
                )
            })
            .collect::<Result<Vec<u128>>>()?;

        let expected_alpha: Vec<u128> = vec![
            1111111111111,
            625000000000,
            400000000000,
            277777777777,
            204081632653,
            156250000000,
            123456790123,
            100000000000,
            82644628099,
            69444444444,
            59171597633,
            51020408163,
            44444444444,
            39062500000,
            34602076124,
            30864197530,
            27700831024,
            25000000000,
            22675736961,
            20661157024,
            18903591682,
        ];

        expected_alpha
            .iter()
            .zip(actual_alpha.clone())
            .for_each(|(expected, actual)| assert_eq!(&actual, expected));

        // Test Beta

        let actual_beta = gamma_s
            .iter()
            .map(|g_s| {
                compute_beta(
                    *g_s as u128,
                    gamma_m,
                    omega_m,
                    price_factor,
                    DEFAULT_DECIMALS_BETA,
                )
            })
            .collect::<Result<Vec<u128>>>()?;

        let expected_beta: Vec<u128> = vec![
            46666666666666666,
            35000000000000000,
            28000000000000000,
            23333333333333333,
            20000000000000000,
            17500000000000000,
            15555555555555555,
            14000000000000000,
            12727272727272727,
            11666666666666666,
            10769230769230769,
            10000000000000000,
            9333333333333333,
            8750000000000000,
            8235294117647058,
            7777777777777777,
            7368421052631578,
            7000000000000000,
            6666666666666666,
            6363636363636363,
            6086956521739130,
        ];

        expected_beta
            .iter()
            .zip(actual_beta.clone())
            .for_each(|(expected, actual)| assert_eq!(&actual, expected));

        Ok(())
    }

    #[test]
    fn test_compute_alpha_abs_with_pf_1() -> Result<()> {
        let gamma_s = vec![
            30000,  //
            40000,  //
            50000,  //
            60000,  //
            70000,  //
            80000,  //
            90000,  //
            100000, //
            110000, //
            120000, //
            130000, //
            140000, //
            150000, //
            160000, //
            170000, //
            180000, //
            190000, //
            200000, //
            210000, //
            220000, //
            230000, //
        ];

        // Test when left < gamma_m
        let gamma_m = 900000000000000;
        let omega_m = 200000000000000;
        let price_factor = 1;

        let actual_alpha = gamma_s
            .iter()
            .map(|g_s| {
                compute_alpha_abs_with_decimals(
                    *g_s as u128,
                    gamma_m,
                    omega_m,
                    price_factor,
                    DEFAULT_DECIMALS_ALPHA,
                )
            })
            .collect::<Result<Vec<u128>>>()?;

        let expected_alpha: Vec<u128> = vec![
            1555555555555,
            875000000000,
            560000000000,
            388888888888,
            285714285714,
            218750000000,
            172839506172,
            140000000000,
            115702479338,
            97222222222,
            82840236686,
            71428571428,
            62222222222,
            54687500000,
            48442906574,
            43209876543,
            38781163434,
            35000000000,
            31746031746,
            28925619834,
            26465028355,
        ];

        expected_alpha
            .iter()
            .zip(actual_alpha.clone())
            .for_each(|(expected, actual)| assert_eq!(&actual, expected));

        // Test Beta

        let actual_beta = gamma_s
            .iter()
            .map(|g_s| {
                compute_beta(
                    *g_s as u128,
                    gamma_m,
                    omega_m,
                    price_factor,
                    DEFAULT_DECIMALS_BETA,
                )
            })
            .collect::<Result<Vec<u128>>>()?;

        let expected_beta: Vec<u128> = vec![
            53333333333333333,
            40000000000000000,
            32000000000000000,
            26666666666666666,
            22857142857142857,
            20000000000000000,
            17777777777777777,
            16000000000000000,
            14545454545454545,
            13333333333333333,
            12307692307692307,
            11428571428571428,
            10666666666666666,
            10000000000000000,
            9411764705882352,
            8888888888888888,
            8421052631578947,
            8000000000000000,
            7619047619047619,
            7272727272727272,
            6956521739130434,
        ];

        expected_beta
            .iter()
            .zip(actual_beta.clone())
            .for_each(|(expected, actual)| assert_eq!(&actual, expected));

        Ok(())
    }
}
