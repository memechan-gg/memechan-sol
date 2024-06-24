use anchor_lang::account;
use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[account]
pub struct ChanSwap {
    pub chan_sol_price_num: u64,
    pub chan_sol_price_denom: u64,
    pub chan_vault: Pubkey,
}

impl ChanSwap {
    pub const CHAN_SWAP_PREFIX: &'static [u8; 9] = b"chan_swap";
    pub const SIGNER_PDA_PREFIX: &'static [u8; 16] = b"chan_swap_signer";

    pub fn space() -> usize {
        let discriminant = 8;
        let chan_sol_price_num = 8;
        let chan_sol_price_denom = 8;
        let chan_vault = 32;

        discriminant + chan_sol_price_num + chan_sol_price_denom + chan_vault
    }
}
