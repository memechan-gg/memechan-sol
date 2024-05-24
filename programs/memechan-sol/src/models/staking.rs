use crate::{
    math::{Decimal, TryAdd, TryDiv, TryMul, TryRound},
    vesting::VestingConfig,
};
use anchor_lang::prelude::*;
use std::mem;

#[account]
pub struct StakingPool {
    pub pool: Pubkey,
    pub meme_vault: Pubkey,
    pub meme_mint: Pubkey,
    pub lp_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub quote_vault: Pubkey,
    pub vesting_config: VestingConfig,
    pub raydium_fees: RaydiumFees,
    pub stakes_total: u64,
    pub fees_x_total: u64,
    pub fees_y_total: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct RaydiumFees {
    pub last_cum_quote_fees: u64,
    pub last_cum_meme_fees: u64,
}

impl StakingPool {
    pub const POOL_PREFIX: &'static [u8; 12] = b"staking_pool";
    pub const SIGNER_PDA_PREFIX: &'static [u8; 7] = b"staking";

    pub fn space() -> usize {
        let discriminant = 8;
        let pool = 32;
        let meme_vault = 32;
        let meme_mint = 32;
        let lp_vault = 32;
        let lp_mint = 32;
        let quote_vault = 32;
        let vesting_config = mem::size_of::<VestingConfig>();
        let last_cum_quote_fees = 8;
        let last_cum_meme_fees = 8;
        let stakes_total = 8;
        let fees_x_total = 8;
        let fees_y_total = 8;

        discriminant
            + pool
            + meme_vault
            + meme_mint
            + lp_vault
            + lp_mint
            + quote_vault
            + vesting_config
            + last_cum_quote_fees
            + last_cum_meme_fees
            + stakes_total
            + fees_x_total
            + fees_y_total
    }
}

impl StakingPool {
    pub fn compute_fee_ratio_and_update(
        &mut self,
        meme_balance: u64,
        acc_meme_fee: u64,
        quote_balance: u64,
        acc_quote_fee: u64,
    ) -> Result<Decimal> {
        let delta_quote_fee = acc_quote_fee - self.raydium_fees.last_cum_quote_fees;
        let delta_meme_fee = acc_meme_fee - self.raydium_fees.last_cum_meme_fees;

        self.raydium_fees.last_cum_quote_fees = acc_quote_fee;
        self.raydium_fees.last_cum_meme_fees = acc_meme_fee;

        arithmetic_fee_ratio(meme_balance, delta_meme_fee, quote_balance, delta_quote_fee)
    }
}

pub fn lp_tokens_to_burn(fee_ratio: Decimal, lp_tokens_owned: u64) -> Result<u64> {
    fee_ratio
        .try_mul(Decimal::from(lp_tokens_owned))?
        .try_round()
}

pub fn arithmetic_fee_ratio(
    reserve_meme: u64,
    cumulated_fees_meme: u64,
    reserve_quote: u64,
    cumulated_fees_quote: u64,
) -> Result<Decimal> {
    token_fee_ratio(reserve_meme, cumulated_fees_meme)?
        .try_add(token_fee_ratio(reserve_quote, cumulated_fees_quote)?)?
        .try_div(Decimal::from(2 as u64))
}

pub fn token_fee_ratio(reserve_balance: u64, cumulated_fees: u64) -> Result<Decimal> {
    Decimal::from(cumulated_fees).try_div(Decimal::from(reserve_balance))
}

// mod tests {
//     use crate::math::{ScaledVal, U192};

//     use super::*;

//     #[test]
//     fn test_cumulated_lp_withdrawal() -> Result<()> {
//         // (reserve_meme, reserve_quote, acc_meme_fee, acc_quote_fee)
//         let data = vec![
//             (50_000_000, 50_000_000, 0, 0),
//             (49_999_905, 50_000_095, 0, 5),
//             (49_999_905, 50_000_095, 5, 5),
//             (49_999_905, 50_000_095, 20_364, 5),
//             (49_352_694, 50_656_249, 20_364, 34_539),
//             (295_748_947, 297_789_037, 20_364, 53_977),
//             (295_742_783, 297_782_873, 37_189, 53_977),
//             (294_929_764, 298_601_054, 37_189, 97_113),
//             (294_086_138, 299_450_604, 37_189, 142_013),
//             (293_653_772, 299_884_141, 37_189, 165_023),
//             (293_019_492, 300_529_578, 37_189, 199_092),
//             (293_019_492, 300_529_578, 54_352, 199_092),
//             (293_015_318, 300_525_405, 97_466, 199_092),
//             (292_377_891, 301_173_377, 97_466, 233_385),
//             (292_375_137, 301_170_623, 105_316, 233_385),
//             (292_374_485, 301_169_971, 138_540, 233_385),
//             (292_374_485, 301_169_971, 180_785, 233_385),
//             (292_166_643, 301_384_225, 180_785, 244_661),
//             (292_159_456, 301_377_038, 194_571, 244_661),
//             (291_811_854, 301_736_056, 194_571, 263_557),
//             (291_809_194, 301_733_396, 220_851, 263_557),
//             (291_809_194, 301_733_396, 264_659, 263_557),
//         ];

//         let expected_arithmetic_fees = vec![
//             Decimal::from_scaled_val(0),
//             Decimal::from_scaled_val(49999905000),
//             Decimal::from_scaled_val(100000000000),
//             Decimal::from_scaled_val(203690386821735),
//             Decimal::from_scaled_val(547226417004325),
//             Decimal::from_scaled_val(125057444306960),
//             Decimal::from_scaled_val(153505364291796),
//             Decimal::from_scaled_val(225660504892121),
//             Decimal::from_scaled_val(300350654331858),
//             Decimal::from_scaled_val(338465761834814),
//             Decimal::from_scaled_val(394693519344115),
//             Decimal::from_scaled_val(423979966950133),
//             Decimal::from_scaled_val(497555420159188),
//             Decimal::from_scaled_val(554137675838306),
//             Decimal::from_scaled_val(567567323012042),
//             Decimal::from_scaled_val(624386106895006),
//             Decimal::from_scaled_val(696630783101960),
//             Decimal::from_scaled_val(715282288288747),
//             Decimal::from_scaled_val(738892859060308),
//             Decimal::from_scaled_val(770118680990106),
//             Decimal::from_scaled_val(815154994363442),
//             Decimal::from_scaled_val(890217742386385),
//         ];

//         // (reserve_meme, reserve_quote, acc_meme_fee, acc_quote_fee)
//         let arithmetic_fee_ratios = data
//             .iter()
//             .map(|row| arithmetic_fee_ratio(row.0, row.2, row.1, row.3))
//             .collect::<Result<Vec<Decimal>>>()?;

//         expected_arithmetic_fees
//             .iter()
//             .zip(arithmetic_fee_ratios.clone())
//             .for_each(|(expected, actual)| assert_eq!(&actual, expected));

//         let lp_tokens_owned = vec![
//             1_000_000_000,
//             1_000_000_000,
//             1_000_000_000,
//             1_000_000_000,
//             1_000_000_000,
//             1_000_000_000,
//             999_874_942,
//             999_846_494,
//             999_774_367,
//             999_699_695,
//             999_661_610,
//             999_661_610,
//             999_576_163,
//             999_502_614,
//             999_446_098,
//             999_432_715,
//             999_432_715,
//             999_432_715,
//             999_285_123,
//             999_285_123,
//             999_230_432,
//             999_230_432,
//         ];

//         let expected_cumulated_lp_withdrawals = vec![
//             0, 50, 100, 203_690, 547_226, 125_057, 153_486, 225_626, 300_283, 338_364, 394_560,
//             423_836, 497_345, 553_862, 567_253, 624_032, 696_236, 714_877, 738_365, 769_568,
//             814_528, 889_533,
//         ];

//         let cumulated_lp_withdrawals = arithmetic_fee_ratios
//             .iter()
//             .zip(lp_tokens_owned.clone())
//             .map(|row| cumulated_lp_withdrawal(row.0, row.1))
//             .collect::<Result<Vec<Decimal>>>()?;

//         expected_cumulated_lp_withdrawals
//             .iter()
//             .zip(cumulated_lp_withdrawals.clone())
//             .for_each(|(expected, actual)| assert_eq!(&actual.try_round().unwrap(), expected));

//         let expected_lp_tokens_to_withdraw = vec![
//             0, 50, 100, 203_690, 547_226, 125_057, 153_486, 225_626, 300_283, 338_364, 394_560,
//             423_836, 497_345, 553_862, 567_253, 624_032, 696_236, 714_877, 738_365, 769_568,
//             814_528, 889_533,
//         ];

//         let lp_tokens_to_withdraw_ = cumulated_lp_withdrawals
//             .iter()
//             .map(|cum_lp_withdrawal| lp_tokens_to_withdraw(cum_lp_withdrawal, 0))
//             .collect::<Result<Vec<u64>>>()?;

//         expected_lp_tokens_to_withdraw
//             .iter()
//             .zip(lp_tokens_to_withdraw_.clone())
//             .for_each(|(expected, actual)| assert_eq!(&actual, expected));

//         Ok(())
//     }
// }
