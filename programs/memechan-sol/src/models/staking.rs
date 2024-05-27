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
    pub raydium_amm: Pubkey,
    pub vesting_config: VestingConfig,
    pub raydium_fees: RaydiumAmmFees,
    pub stakes_total: u64,
    pub fees_x_total: u64,
    pub fees_y_total: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct RaydiumAmmFees {
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
        let raydium_amm = 32;
        let vesting_config = mem::size_of::<VestingConfig>();
        let raydium_fees = mem::size_of::<RaydiumAmmFees>();
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
            + raydium_amm
            + vesting_config
            + raydium_fees
            + stakes_total
            + fees_x_total
            + fees_y_total
    }
}

impl StakingPool {
    pub fn compute_fee_ratio(
        &mut self,
        meme_balance: u64,
        acc_meme_fee: u64,
        quote_balance: u64,
        acc_quote_fee: u64,
    ) -> Result<Decimal> {
        let fee_ratio = compute_fee_ratio(
            self.raydium_fees.last_cum_quote_fees,
            self.raydium_fees.last_cum_meme_fees,
            meme_balance,
            acc_meme_fee,
            quote_balance,
            acc_quote_fee,
        )?;

        Ok(fee_ratio)
    }
}

pub fn compute_fee_ratio(
    last_cum_quote_fees: u64,
    last_cum_meme_fees: u64,
    meme_balance: u64,
    acc_meme_fee: u64,
    quote_balance: u64,
    acc_quote_fee: u64,
) -> Result<Decimal> {
    let delta_quote_fee = acc_quote_fee - last_cum_quote_fees;
    let delta_meme_fee = acc_meme_fee - last_cum_meme_fees;

    arithmetic_fee_ratio(meme_balance, delta_meme_fee, quote_balance, delta_quote_fee)
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

mod tests {
    use super::*;

    #[test]
    fn test_cumulated_lp_withdrawal() -> Result<()> {
        // (reserve_meme, reserve_quote, delta_acc_meme_fee, delta_acc_quote_fee)
        let data = vec![
            (50_000_000, 50_000_000, 0, 0),
            (49_999_905, 50_000_095, 0, 5),
            (49_999_905, 50_000_095, 5, 0),
            (49_999_905, 50_000_095, 20359, 0),
            (49_352_694, 50_656_249, 0, 34534),
            (295_748_947, 297_789_037, 0, 19438),
            (295_742_783, 297_782_873, 16825, 0),
            (294_929_764, 298_601_054, 0, 43136),
            (294_086_138, 299_450_604, 0, 44900),
            (293_653_772, 299_884_141, 0, 23010),
            (293_019_492, 300_529_578, 0, 34069),
            (293_019_492, 300_529_578, 17163, 0),
            (293_015_318, 300_525_405, 43113, 0),
            (292_377_891, 301_173_377, 0, 34293),
            (292_375_137, 301_170_623, 7850, 0),
            (292_374_485, 301_169_971, 33224, 0),
            (292_374_485, 301_169_971, 42245, 0),
            (292_166_643, 301_384_225, 0, 11277),
            (292_159_456, 301_377_038, 13786, 0),
            (291_811_854, 301_736_056, 0, 18896),
            (291_809_194, 301_733_396, 26280, 0),
            (291_809_194, 301_733_396, 43808, 0),
        ];

        let expected_arithmetic_fees = vec![
            Decimal::from_scaled_val(0),
            Decimal::from_scaled_val(49999905000),
            Decimal::from_scaled_val(50000095000),
            Decimal::from_scaled_val(203590386821734),
            Decimal::from_scaled_val(340866138746277),
            Decimal::from_scaled_val(32637198796542),
            Decimal::from_scaled_val(28445326424077),
            Decimal::from_scaled_val(72230153614929),
            Decimal::from_scaled_val(74970628544799),
            Decimal::from_scaled_val(38364816364197),
            Decimal::from_scaled_val(56681608889757),
            Decimal::from_scaled_val(29286447606017),
            Decimal::from_scaled_val(73567826239036),
            Decimal::from_scaled_val(56932323071836),
            Decimal::from_scaled_val(13424534111462),
            Decimal::from_scaled_val(56817543432355),
            Decimal::from_scaled_val(72244676206954),
            Decimal::from_scaled_val(18708676607078),
            Decimal::from_scaled_val(23593280513227),
            Decimal::from_scaled_val(31312134602833),
            Decimal::from_scaled_val(45029424261389),
            Decimal::from_scaled_val(75062748022942),
        ];

        // (reserve_meme, reserve_quote, acc_meme_fee, acc_quote_fee)
        let arithmetic_fee_ratios = data
            .iter()
            .map(|row| arithmetic_fee_ratio(row.0, row.2, row.1, row.3))
            .collect::<Result<Vec<Decimal>>>()?;

        expected_arithmetic_fees
            .iter()
            .zip(arithmetic_fee_ratios.clone())
            .for_each(|(expected, actual)| assert_eq!(&actual, expected));

        let lp_tokens_owned = vec![
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            1_000_000_000,
            999_874_942,
            999_846_494,
            999_774_367,
            999_699_695,
            999_661_610,
            999_661_610,
            999_576_163,
            999_502_614,
            999_446_098,
            999_432_715,
            999_432_715,
            999_432_715,
            999_285_123,
            999_285_123,
            999_230_432,
            999_230_432,
        ];

        let expected_lp_tokens_to_burn = vec![
            0, 50, 50, 203590, 340866, 32637, 28442, 72219, 74954, 38353, 56662, 29277, 73537,
            56904, 13417, 56785, 72204, 18698, 23576, 31290, 44995, 75005,
        ];

        let lp_tokens_to_burn = arithmetic_fee_ratios
            .iter()
            .zip(lp_tokens_owned.clone())
            .map(|row| lp_tokens_to_burn(*row.0, row.1))
            .collect::<Result<Vec<u64>>>()?;

        expected_lp_tokens_to_burn
            .iter()
            .zip(lp_tokens_to_burn.clone())
            .for_each(|(expected, actual)| assert_eq!(&actual, expected));

        Ok(())
    }
}
