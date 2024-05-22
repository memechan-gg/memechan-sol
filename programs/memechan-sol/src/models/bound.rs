use crate::{consts::DECIMALS_S, err::AmmError, models::CheckedMath};
use anchor_lang::prelude::*;
use num_integer::Roots;
use solana_program::pubkey::Pubkey;
use std::{cmp::min, mem};

use super::{fees::Fees, Reserve, SwapAmount};

#[account]
#[derive(Default)]
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
    pub quote: u64,
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

        let max_delta_s = (p.gamma_s * p.decimals.quote) - s_t0;

        let admin_fee_in = self.fees.get_fee_in_amount(delta_s).unwrap();
        let is_max = delta_s - admin_fee_in >= max_delta_s;

        let net_delta_s = min(delta_s - admin_fee_in, max_delta_s);

        let delta_m = if is_max {
            m_t0
        } else {
            self.compute_delta_m(s_t0, s_t0 + net_delta_s)?
        };

        let admin_fee_out = self.fees.get_fee_out_amount(delta_m).unwrap();
        let net_delta_m = delta_m - admin_fee_out;

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
            s_b
        } else {
            self.compute_delta_s(s_b, net_delta_m)?
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

    pub fn compute_delta_m(&self, s_a: u64, s_b: u64) -> Result<u64> {
        let s_a = s_a as u128;
        let s_b = s_b as u128;

        let alpha_abs = self.config.alpha_abs;
        let beta = self.config.beta;
        let alpha_decimals = self.config.decimals.alpha;
        let beta_decimals = self.config.decimals.beta;

        match delta_m1_strategy(alpha_abs, beta, alpha_decimals, beta_decimals, s_a, s_b) {
            Some(delta_m) => return Ok(delta_m as u64),
            None => {
                match delta_m2_strategy(alpha_abs, beta, alpha_decimals, beta_decimals, s_a, s_b) {
                    Some(delta_m) => return Ok(delta_m as u64),
                    None => return Err(error!(AmmError::MathOverflow)),
                }
            }
        }
    }

    pub fn compute_delta_s(&self, s_b: u64, delta_m: u64) -> Result<u64> {
        let s_b = s_b as u128;
        let delta_m = delta_m as u128;

        let alpha_abs = self.config.alpha_abs;
        let beta = self.config.beta;
        let alpha_decimals = self.config.decimals.alpha;
        let beta_decimals = self.config.decimals.beta;

        match delta_s1_strategy(alpha_abs, beta, alpha_decimals, beta_decimals, s_b, delta_m) {
            Some(delta_s) => Ok(delta_s as u64),
            None => match delta_s2_strategy(
                alpha_abs,
                beta,
                alpha_decimals,
                beta_decimals,
                s_b,
                delta_m,
            ) {
                Some(delta_s) => Ok(delta_s as u64),
                None => Err(error!(AmmError::MathOverflow)),
            },
        }
    }

    fn balances(&self) -> (u64, u64) {
        (self.meme_reserve.tokens, self.quote_reserve.tokens)
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

fn delta_s1_strategy(
    alpha_abs: u128,
    beta: u128,
    alpha_decimals: u128,
    beta_decimals: u128,
    s_b: u128,
    delta_m: u128,
) -> Option<u128> {
    // a1 = 2 * beta * alpha_decimals * DECIMALS_S - 2 * alpha_abs * s_b * beta_decimals;
    let a1 = (2 * beta)
        .checked_mul(alpha_decimals)
        .checked_mul(DECIMALS_S)
        .checked_sub_((2 * alpha_abs).checked_mul(s_b).checked_mul(beta_decimals));

    if let None = a1 {
        return None;
    }

    // b1 = beta_decimals * beta_decimals * DECIMALS_S;
    let b1 = beta_decimals.checked_pow(2).checked_mul(DECIMALS_S);

    if let None = b1 {
        return None;
    }

    // c1 = 8 * delta_m * alpha_abs;
    let c1 = (8 * alpha_abs).checked_mul(delta_m);

    if let None = c1 {
        return None;
    }

    // a = (a1.pow(2) * alpha_decimals + c1 * b1.pow(2)).sqrt();
    let a = a1
        .checked_pow(2)
        .checked_mul(alpha_decimals)
        .checked_add_(b1.checked_pow(2).checked_mul_(c1))
        .sqrt();

    if let None = a {
        return None;
    }

    // b = (alpha_decimals * b1.pow(2)).sqrt();
    let b = b1.checked_pow(2).checked_mul(alpha_decimals).sqrt();

    if let None = b {
        return None;
    }

    // u = 2 * beta * alpha_decimals * DECIMALS_S - 2 * alpha_abs * s_b * beta_decimals;
    let u = (2 * beta)
        .checked_mul(alpha_decimals)
        .checked_mul(DECIMALS_S)
        .checked_sub_((2 * alpha_abs).checked_mul(s_b).checked_mul(beta_decimals));

    if let None = u {
        return None;
    }

    // v = alpha_decimals * beta_decimals * DECIMALS_S;
    let v = alpha_decimals
        .checked_mul(beta_decimals)
        .checked_mul(DECIMALS_S);

    if let None = v {
        return None;
    }

    // num = (a * v - u * b) * DECIMALS_S * alpha_decimals;
    let num = a
        .checked_mul_(v)
        .checked_sub_(u.checked_mul_(b))
        .checked_mul(DECIMALS_S)
        .checked_mul(alpha_decimals);

    if let None = num {
        return None;
    }

    // denom = (2 * alpha_abs) * (b * v);
    let denom = b.checked_mul_(v).checked_mul(2 * alpha_abs);

    if let None = denom {
        return None;
    }

    // delta_s = num / denom
    num.checked_div_(denom)
}

fn multiply_until_overflow(nums: Vec<u128>) -> (u128, Vec<u128>) {
    fn helper(nums: &[u128], current_product: u128) -> (u128, Vec<u128>) {
        if nums.is_empty() {
            return (current_product, vec![]);
        }

        let next = nums[0];
        if let Some(result) = current_product.checked_mul(next) {
            helper(&nums[1..], result)
        } else {
            (current_product, nums.to_vec())
        }
    }

    if nums.is_empty() {
        return (1, vec![]);
    }

    helper(&nums, 1)
}

fn delta_s2_strategy(
    alpha_abs: u128,
    beta: u128,
    alpha_decimals: u128,
    beta_decimals: u128,
    s_b: u128,
    delta_m: u128,
) -> Option<u128> {
    // u = 2 * beta * alpha_decimals * DECIMALS_S - 2 * alpha_abs * s_b * beta_decimals;
    let u = (2 * beta)
        .checked_mul(alpha_decimals)
        .checked_mul(DECIMALS_S)
        .checked_sub_((2 * alpha_abs).checked_mul(s_b).checked_mul(beta_decimals));

    if let None = u {
        return None;
    }

    let w = (8 * alpha_abs).checked_mul(delta_m);

    if let None = w {
        return None;
    }

    // First = a / (D_beta * sqrt(D_alpha) * 2 * alpha_abs)
    // Where, a = sqrt( u^2 * D_alpha + wv^2 )
    // <=> a = u * sqrt(D_alpha) + sqrt( w * D_alpha^2 * D_beta^2 * D_S^2)
    // In relation to the second term we want to maximize precision without risking overflow
    // therefore we use `multiply_until_overflow` which multiply the terms inside the square root until
    // before it hits overflow

    let first_num_1 = u.checked_mul(alpha_decimals.sqrt());

    if let None = first_num_1 {
        return None;
    }

    let (maxed_elem, remaining_elems) = multiply_until_overflow(vec![
        w.unwrap(),
        alpha_decimals,
        beta_decimals,
        DECIMALS_S,
        alpha_decimals,
        beta_decimals,
        DECIMALS_S,
    ]);

    // Once we hit a maxed value we scale it down with the square root
    let maxed_elem_sqrt = Some(maxed_elem.sqrt());

    // And multiply against the remaining factor. The remaining factor is the product of
    // all the square roots of the remaining elements
    let remaining_factor = remaining_elems
        .iter()
        .try_fold(1u128, |acc, &num| acc.checked_mul(num.sqrt()));

    let first_num_2 = maxed_elem_sqrt.checked_mul_(remaining_factor);

    if let None = first_num_2 {
        return None;
    }

    let first_num = first_num_1.checked_add_(first_num_2);
    let first_denom = beta_decimals
        .checked_mul(alpha_decimals.sqrt())
        .checked_mul(2 * alpha_abs);

    let first_term = first_num.checked_div_(first_denom);

    if let None = first_term {
        return None;
    }

    let second_term = beta
        .checked_mul(DECIMALS_S)
        .checked_div(beta_decimals)
        .checked_mul(alpha_decimals)
        .checked_div(alpha_abs);

    if let None = second_term {
        return None;
    }

    first_term.checked_sub_(second_term).checked_add(s_b)
}

fn delta_m2_strategy(
    alpha_abs: u128,
    beta: u128,
    alpha_decimals: u128,
    beta_decimals: u128,
    s_a: u128,
    s_b: u128,
) -> Option<u128> {
    println!("STRATEGY 2");
    let left = (beta * 2)
        .checked_mul(DECIMALS_S)
        .checked_mul(alpha_decimals)
        .checked_mul(s_b - s_a);

    if let None = left {
        return None;
    }

    let right = alpha_abs
        .checked_mul(beta_decimals)
        .checked_mul_(s_b.checked_pow(2).checked_sub_(s_a.checked_pow(2)));

    if let None = right {
        return None;
    }

    let denom = (2 * alpha_decimals)
        .checked_mul(beta_decimals)
        .checked_mul_(DECIMALS_S.checked_pow(2));

    if let None = denom {
        return None;
    }

    left.checked_sub_(right).checked_div_(denom)
}

fn delta_m1_strategy(
    alpha_abs: u128,
    beta: u128,
    alpha_decimals: u128,
    beta_decimals: u128,
    s_a: u128,
    s_b: u128,
) -> Option<u128> {
    let left_num = (s_b.checked_sub(s_a)).checked_mul(beta);

    if let None = left_num {
        return None;
    }

    let left_denom = beta_decimals.checked_mul(DECIMALS_S);

    if let None = left_denom {
        return None;
    }

    let left = left_num.checked_div_(left_denom);

    if let None = left {
        return None;
    }

    let right = s_b
        .checked_pow(2)
        .checked_sub_(s_a.checked_pow(2))
        .checked_div_(DECIMALS_S.checked_pow(2))
        .checked_mul(alpha_abs)
        .checked_div(2 * alpha_decimals);

    if let None = right {
        return None;
    }

    // println!("left: {:?}", left.unwrap());
    // println!("right: {:?}", right.unwrap());
    // println!("left- right: {:?}", left.checked_sub_(right).unwrap());

    // panic!();

    left.checked_sub_(right)
}

#[cfg(test)]
mod tests {
    use proptest::*;
    use std::fs::File;

    use csv::ReaderBuilder;

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

    #[test]
    fn test_compute_delta_m() -> Result<()> {
        let filename = "../../data/delta_m.csv";
        let expected_delta_ms = read_csv_column(filename);

        let gamma_s: u128 = 30_000;
        let gamma_m: u128 = 900_000_000_000_000;
        let omega_m: u128 = 200_000_000_000_000;
        let price_factor = 2;

        let (alpha, alpha_decimals) = compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor)?;
        let beta = compute_beta(gamma_s, gamma_m, omega_m, price_factor, alpha_decimals)?;

        let pool = BoundPool {
            config: Config {
                alpha_abs: alpha,
                beta,
                price_factor,
                gamma_s: gamma_s as u64,
                gamma_m: gamma_m as u64,
                omega_m: omega_m as u64,
                decimals: Decimals {
                    alpha: DEFAULT_DECIMALS_ALPHA,
                    beta: DEFAULT_DECIMALS_BETA,
                    quote: 1_000_000_000,
                },
            },
            ..Default::default()
        };

        let mut s_a = 0;

        for expected in expected_delta_ms.iter() {
            let actual = pool.compute_delta_m(s_a * 1_000_000_000, (s_a + 1) * 1_000_000_000)?;

            assert_eq!(expected, &actual);

            s_a += 1;
        }

        Ok(())
    }

    #[test]
    fn test_compute_delta_m_2() -> Result<()> {
        let filename = "../../data/delta_m_2.csv";
        let expected_delta_ms = read_csv_column(filename);

        let gamma_s: u128 = 10;
        let gamma_m: u128 = 900_000_000_000_000;
        let omega_m: u128 = 200_000_000_000_000;
        let price_factor = 2;

        let (alpha, alpha_decimals) = compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor)?;
        let beta = compute_beta(gamma_s, gamma_m, omega_m, price_factor, alpha_decimals)?;

        let pool = BoundPool {
            config: Config {
                alpha_abs: alpha,
                beta,
                price_factor,
                gamma_s: gamma_s as u64,
                gamma_m: gamma_m as u64,
                omega_m: omega_m as u64,
                decimals: Decimals {
                    alpha: alpha_decimals,
                    beta: alpha_decimals,
                    quote: 1_000_000_000,
                },
            },
            ..Default::default()
        };

        let mut s_a = 0;

        for expected in expected_delta_ms.iter() {
            let actual = pool.compute_delta_m(s_a * 1_000_000_000, (s_a + 1) * 1_000_000_000)?;

            assert_eq!(expected, &actual);

            s_a += 1;
        }

        Ok(())
    }

    #[test]
    fn test_compute_delta_m_3() -> Result<()> {
        let filename = "../../data/delta_m_3.csv";
        let expected_delta_ms = read_csv_column(filename);

        let gamma_s: u128 = 1_000;
        let gamma_m: u128 = 800_000_000_000_000;
        let omega_m: u128 = 200_000_000_000_000;
        let price_factor = 2;

        let (alpha, alpha_decimals) = compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor)?;
        let beta = compute_beta(gamma_s, gamma_m, omega_m, price_factor, alpha_decimals)?;

        let pool = BoundPool {
            config: Config {
                alpha_abs: alpha,
                beta,
                price_factor,
                gamma_s: gamma_s as u64,
                gamma_m: gamma_m as u64,
                omega_m: omega_m as u64,
                decimals: Decimals {
                    alpha: alpha_decimals,
                    beta: alpha_decimals,
                    quote: 1_000_000_000,
                },
            },
            ..Default::default()
        };

        let mut s_a = 0;

        for expected in expected_delta_ms.iter() {
            let actual = pool.compute_delta_m(s_a * 1_000_000_000, (s_a + 1) * 1_000_000_000)?;

            println!("{:?}", actual);

            assert_eq!(expected, &actual);

            s_a += 1;
        }

        Ok(())
    }

    fn read_csv_column(filename: &str) -> Vec<u64> {
        // Open the CSV file
        let file = File::open(filename).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);

        // Read the first column into a vector
        let mut column_data = Vec::new();
        for result in rdr.records() {
            let record = result.unwrap();
            if let Some(column) = record.get(0) {
                let value: u64 = column.parse().unwrap();
                column_data.push(value);
            }
        }

        column_data
    }

    pub fn check_slope_(gamma_m: u128, omega_m: u128, price_factor: u64) -> bool {
        if price_factor as u128 * omega_m >= gamma_m {
            false
        } else {
            true
        }
    }

    pub fn check_intercept_(gamma_m: u128, omega_m: u128, price_factor: u64) -> bool {
        if 2 * gamma_m <= omega_m * (price_factor as u128) {
            false
        } else {
            true
        }
    }

    pub fn check_scale_and_gamma_s_rel(
        gamma_s: u128,
        gamma_m: u128,
        omega_m: u128,
        price_factor: u64,
    ) -> bool {
        let left = omega_m * (price_factor as u128);

        let num = 2 * (gamma_m - left);
        let denom = gamma_s * gamma_s;

        if num <= denom {
            return false;
        }

        let num_scale = compute_scale(num);
        let denom_scale = compute_scale(denom);

        let net_scale = num_scale - denom_scale;

        match net_scale {
            0..=4 => return false,
            5 => return true,
            6 => return true,
            7 => return true,
            8 => return true,
            9 => return true,
            10 => return true,
            11 => return true,
            12 => return true,
            _ => return true,
        }
    }

    proptest! {
        #[test]
        fn successfully_returns_positive_exponent(
            gamma_s in 1_000..10_000_000_u128,
            gamma_m in 100_000_000_u128..900_000_000_000_000_000_u128,
            omega_m in 100_000_000_u128..900_000_000_000_000_000_u128,
            price_factor in 1..4u128
        ) {
            let price_factor = price_factor as u64;

            if check_slope_(gamma_m, omega_m, price_factor) == false {
                return Ok(())
            }

            if check_intercept_(gamma_m, omega_m, price_factor) == false {
                return Ok(())
            }

            if check_scale_and_gamma_s_rel(gamma_s, gamma_m, omega_m, price_factor) == false {
                return Ok(())
            }

            let (alpha, alpha_decimals) = compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor)?;
            let beta = compute_beta(gamma_s, gamma_m, omega_m, price_factor, alpha_decimals)?;

            let pool = BoundPool {
                config: Config {
                    alpha_abs: alpha,
                    beta,
                    price_factor,
                    gamma_s: gamma_s as u64,
                    gamma_m: gamma_m as u64,
                    omega_m: omega_m as u64,
                    decimals: Decimals {
                        alpha: alpha_decimals,
                        beta: alpha_decimals,
                        quote: 1_000_000_000,
                    },
                },
                ..Default::default()
            };

            let mut s_a = 0;

            let delta_s = 1000;

            for _ in (0..gamma_s).step_by(delta_s) {
                let actual = pool.compute_delta_m(s_a * 1_000_000_000, (s_a + delta_s as u64) * 1_000_000_000);

                if actual.is_err() {
                    println!("s_a: {:?}", s_a);
                    println!("s_b: {:?}", s_a + delta_s as u64);
                    println!("gamma_s: {:?}", gamma_s);
                    println!("gamma_m: {:?}", gamma_m);
                    println!("omega_m: {:?}", omega_m);
                    println!("price_factor: {:?}", price_factor);

                    panic!();
                }

                assert!(actual.unwrap() != 0);

                s_a += 1;
            }
        }
    }
}
