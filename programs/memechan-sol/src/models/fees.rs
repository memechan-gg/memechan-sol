use crate::libraries::MulDiv;
use anchor_lang::prelude::*;

pub const MEME_FEE: u64 = 0; // 0%
pub const FEE: u64 = 10_000_000; // 1%
pub const LAUNCH_FEE: u64 = 50_000_000; // 5%
pub const COMMS_FEE: u64 = 50_000_000; // 5%
pub const FEE_PRECISION: u64 = 1_000_000_000;

pub const REFERRER_POINTS_NUMERATOR: u64 = 25_000; // 25%
pub const REFERRER_POINTS_DENOMINATOR: u64 = 100_000;

pub const REFERRER_QUOTE_FEE_NUMERATOR: u64 = 250_000; // 2.5%
pub const REFERRER_QUOTE_FEE_DENOMINATOR: u64 = 10_000_000;

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Fees {
    pub fee_meme_percent: u64,
    pub fee_quote_percent: u64,
}

impl Fees {
    pub fn get_fee_meme_amount(&self, amount: u64) -> Result<u64> {
        get_fee_amount(amount, self.fee_meme_percent)
    }

    pub fn get_fee_quote_amount(&self, amount: u64) -> Result<u64> {
        get_fee_amount(amount, self.fee_quote_percent)
    }
}

pub fn get_fee_amount(x: u64, percent: u64) -> Result<u64> {
    Ok(x.mul_div_ceil(percent, FEE_PRECISION).unwrap())
}
