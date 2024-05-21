pub mod bound;
pub mod fee_distribution;
pub mod fees;
pub mod staked_lp;
pub mod staking;
pub mod target_config;

use anchor_lang::prelude::*;
use num_integer::Roots;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TokenLimit {
    pub mint: Pubkey,
    pub tokens: TokenAmount,
}

#[derive(
    AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd,
)]
pub struct TokenAmount {
    pub amount: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Reserve {
    pub tokens: u64,
    pub mint: Pubkey,
    pub vault: Pubkey,
}

pub struct SwapAmount {
    pub amount_in: u64,
    pub amount_out: u64,
    pub admin_fee_in: u64,
    pub admin_fee_out: u64,
}

#[derive(Clone)]
pub struct OpenBook;

impl anchor_lang::Id for OpenBook {
    fn id() -> Pubkey {
        // Devnet
        solana_program::pubkey!("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj")
        // Mainnet
        // solana_program::pubkey!("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX")
    }
}

pub trait CheckedMath {
    fn checked_add(&self, num: u128) -> Self;

    fn checked_mul(&self, num: u128) -> Self;

    fn checked_sub(&self, num: u128) -> Self;

    fn checked_div(&self, num: u128) -> Self;

    fn checked_pow(&self, num: u32) -> Self;

    fn checked_add_(&self, num: Option<u128>) -> Self;

    fn checked_mul_(&self, num: Option<u128>) -> Self;

    fn checked_sub_(&self, num: Option<u128>) -> Self;

    fn checked_div_(&self, num: Option<u128>) -> Self;

    fn sqrt(&self) -> Self;
}

impl CheckedMath for Option<u128> {
    fn checked_add(&self, num: u128) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_add(num),
        }
    }

    fn checked_mul(&self, num: u128) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_mul(num),
        }
    }

    fn checked_sub(&self, num: u128) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_sub(num),
        }
    }

    fn checked_div(&self, num: u128) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_div(num),
        }
    }

    fn checked_pow(&self, num: u32) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_pow(num),
        }
    }

    fn checked_add_(&self, num: Option<u128>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_add(num__),
            },
        }
    }

    fn checked_sub_(&self, num: Option<u128>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_sub(num__),
            },
        }
    }

    fn checked_mul_(&self, num: Option<u128>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_mul(num__),
            },
        }
    }

    fn checked_div_(&self, num: Option<u128>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_div(num__),
            },
        }
    }

    fn sqrt(&self) -> Self {
        match self {
            None => None,
            Some(num_) => Some((*num_).sqrt()),
        }
    }
}
