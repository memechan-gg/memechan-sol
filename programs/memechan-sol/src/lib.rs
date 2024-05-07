pub mod consts;
pub mod endpoints;
pub mod err;
pub mod libraries;
pub mod models;
pub mod raydium;
pub mod vesting;

use anchor_lang::prelude::*;
use core as core_;

use endpoints::*;

pub const RAYDIUM_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8"); // Devnet

declare_id!("3LpdC7WHSrw2d6mWm3Enfvpzy1u5zoHkysyH1WxdmpPB");

pub mod admin {
    use anchor_lang::prelude::declare_id;
    declare_id!("8vBA2MzaQdt3UWimSkx1J4m2zMgp8A2iwtRKzXVurXP2");
}

#[program]
pub mod memechan_sol {
    use super::*;

    pub fn new(ctx: Context<New>) -> Result<()> {
        new::handle(ctx)
    }

    pub fn swap_x(
        ctx: Context<SwapCoinX>,
        coin_in_amount: u64,
        coin_y_min_value: u64,
    ) -> Result<()> {
        swap_x::handle(ctx, coin_in_amount, coin_y_min_value)
    }

    pub fn swap_y(
        ctx: Context<SwapCoinY>,
        coin_in_amount: u64,
        coin_x_min_value: u64,
    ) -> Result<()> {
        swap_y::handle(ctx, coin_in_amount, coin_x_min_value)
    }

    pub fn go_live<'info>(ctx: Context<'_, '_, '_, 'info, GoLive<'info>>, nonce: u8) -> Result<()> {
        go_live::handle(ctx, nonce)
    }

    pub fn add_fees<'info>(ctx: Context<'_, '_, '_, 'info, AddFees<'info>>) -> Result<()> {
        add_fees::handle(ctx)
    }

    pub fn unstake(ctx: Context<Unstake>, release_amount: u64) -> Result<()> {
        unstake::handle(ctx, release_amount)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        withdraw_fees::handle(ctx)
    }

    pub fn bound_merge_tickets(ctx: Context<BoundMergeTickets>) -> Result<()> {
        bound_merge_tickets::handle(ctx)
    }

    pub fn staking_merge_tickets(ctx: Context<StakingMergeTickets>) -> Result<()> {
        staking_merge_tickets::handle(ctx)
    }

    pub fn close_ticket(ctx: Context<CloseTicket>) -> Result<()> {
        close_ticket::handle(ctx)
    }
}

//
// // We keep track of how much each address ownes of coin_m
// add_from_token_acc(pool, swap_amount, sender(ctx));
// staked_lp
// }
//
// fun new_fees(
// fee_in_percent: u256,
// fee_out_percent: u256,
// ): Fees {
// fees::new(fee_in_percent, fee_out_percent)
// }
//
