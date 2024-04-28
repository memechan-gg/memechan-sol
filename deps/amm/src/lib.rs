#![allow(clippy::result_large_err)]

pub mod consts;
pub mod endpoints;
pub mod err;
pub mod misc;
pub mod models;
pub mod prelude;

use crate::endpoints::*;
use crate::prelude::*;

// TODO: conditionally compile this based on feature "dev"
declare_id!("8MGDvwp4baAJ4y7rv3wYVw7g1cWVXehuhVoTes68jGxn");

#[program]
pub mod amm {
    use super::*;

    /// # Important
    /// This endpoint requires different accounts based on whether the program
    /// is compiled with the "dev" feature.
    pub fn create_program_toll(_ctx: Context<CreateProgramToll>) -> Result<()> {
        Ok(())
    }

    /// # Important
    /// This endpoint requires different accounts based on whether the program
    /// is compiled with the "dev" feature.
    pub fn create_discount_settings(_ctx: Context<CreateDiscountSettings>) -> Result<()> {
        Ok(())
    }

    pub fn create_pool(_ctx: Context<CreatePool>, _amplifier: u64) -> Result<()> {
        Ok(())
    }

    pub fn put_discount(
        _ctx: Context<PutDiscount>,
        _user: Pubkey,
        _discount_amount: Permillion,
        _valid_until: Slot,
    ) -> Result<()> {
        Ok(())
    }

    pub fn set_pool_swap_fee(_ctx: Context<SetPoolSwapFee>, _fee: Permillion) -> Result<()> {
        Ok(())
    }

    pub fn deposit_liquidity<'info>(
        _ctx: Context<'_, '_, '_, 'info, DepositLiquidity<'info>>,
        _max_amount_tokens: Vec<TokenLimit>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn redeem_liquidity<'info>(
        _ctx: Context<'_, '_, '_, 'info, RedeemLiquidity<'info>>,
        _lp_tokens_to_burn: TokenAmount,
        _min_amount_tokens: Vec<TokenLimit>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn swap<'info>(
        _ctx: Context<'_, '_, '_, 'info, Swap<'info>>,
        _sell: TokenAmount,
        _min_buy: TokenAmount,
    ) -> Result<()> {
        Ok(())
    }
}
