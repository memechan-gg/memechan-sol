use crate::libraries::MulDiv;
use anchor_lang::prelude::*;

pub const FEE: u64 = 1_000_000; // 0.1%
pub const LAUNCH_FEE: u64 = 50_000_000; // 5%
pub const PRECISION: u64 = 1_000_000_000;

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Fees {
    pub fee_in_percent: u64,
    pub fee_out_percent: u64,
}

impl Fees {
    pub fn get_fee_in_amount(&self, amount: u64) -> Result<u64> {
        get_fee_amount(amount, self.fee_in_percent)
    }

    pub fn get_fee_out_amount(&self, amount: u64) -> Result<u64> {
        get_fee_amount(amount, self.fee_out_percent)
    }
}

fn get_fee_amount(x: u64, percent: u64) -> Result<u64> {
    Ok(x.mul_div_ceil(percent, PRECISION).unwrap())
}
