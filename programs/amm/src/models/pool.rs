//! TODO: docs

use crate::prelude::*;
use std::collections::BTreeMap;
use std::mem;

#[derive(Default, Debug, Eq, PartialEq)]
#[account]
pub struct Pool {
    pub admin: Pubkey,
    pub signer: Pubkey,
    pub mint: Pubkey,
    /// The swap fee is divided into a part that goes to the program's owner
    /// and part that goes to liquidity providers via increase in LP token
    /// worth.
    ///
    /// The destination for the former is defined by this key.
    pub program_toll_wallet: Pubkey,
    /// How many reserves in the `reserves` array are initialized (ie. not
    /// having mint of [`Pubkey::default`]).
    pub dimension: u64,
    /// The pool as a maximum reserve size of 4 and can have less reserves
    /// than that. If the pool only has 2 token reserves then, then first two
    /// elements of this array represent those reserves and the other two
    /// elements should have the default value.
    ///
    /// Use [`Pool::reserves`] or [`Pool::reserves_mut`] to access this field,
    /// as those methods filter out the uninitialized reserves.
    pub reserves: [Reserve; 4],
    pub curve: Curve,
    pub swap_fee: Permillion,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Curve {
    ConstProd,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Reserve {
    pub tokens: TokenAmount,
    pub mint: Pubkey,
    pub vault: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TokenLimit {
    pub mint: Pubkey,
    pub tokens: TokenAmount,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct DepositResult {
    /// # Important
    /// This value can be [`None`] if tokens to deposit are so small that LP
    /// tokens don't have enough precision to represent the liquidity.
    pub lp_tokens_to_distribute: Option<TokenAmount>,
    pub tokens_to_deposit: BTreeMap<Pubkey, TokenAmount>,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct RedeemResult {
    pub lp_tokens_to_burn: TokenAmount,
    pub tokens_to_redeem: BTreeMap<Pubkey, TokenAmount>,
}

impl Default for Curve {
    fn default() -> Self {
        Curve::ConstProd
    }
}

impl Pool {
    pub const SIGNER_PDA_PREFIX: &'static [u8; 6] = b"signer";

    pub fn space() -> usize {
        let discriminant = 8;
        let initializer = 32;
        let signer = 32;
        let lp_token_program_fee_wallet = 32;
        let mint = 32;
        let dimension = 8;
        let reserves = mem::size_of::<Reserve>() * 4;
        let curve = mem::size_of::<Curve>();
        let fee = mem::size_of::<Permillion>();

        discriminant
            + initializer
            + signer
            + lp_token_program_fee_wallet
            + mint
            + dimension
            + reserves
            + curve
            + fee
    }
}
