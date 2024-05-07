use anchor_lang::prelude::*;

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
