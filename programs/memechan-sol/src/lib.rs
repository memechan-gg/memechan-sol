pub mod consts;
pub mod endpoints;
pub mod err;
pub mod libraries;
pub mod math;
pub mod models;
pub mod raydium;
pub mod vesting;

use anchor_lang::prelude::*;
use core as core_;

use endpoints::*;

declare_id!("cYsHcSU42XESLPquuN1ga94jm1wVMg11wVcxqvofA3k");

#[program]
pub mod memechan_sol {
    use super::*;

    pub fn new_pool(ctx: Context<NewPool>) -> Result<()> {
        new_pool::handle(ctx)
    }

    pub fn create_metadata(
        ctx: Context<CreateMetadata>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        create_metadata::handle(ctx, name, symbol, uri)
    }

    pub fn get_swap_x_amt(
        ctx: Context<GetSwapXAmt>,
        coin_in_amount: u64,
        coin_y_min_value: u64,
    ) -> Result<()> {
        get_swap_x_amt::handle(ctx, coin_in_amount, coin_y_min_value)
    }

    pub fn swap_x(
        ctx: Context<SwapCoinX>,
        coin_in_amount: u64,
        coin_y_min_value: u64,
    ) -> Result<()> {
        swap_x::handle(ctx, coin_in_amount, coin_y_min_value)
    }

    pub fn get_swap_y_amt(
        ctx: Context<GetSwapYAmt>,
        coin_in_amount: u64,
        coin_x_min_value: u64,
    ) -> Result<()> {
        get_swap_y_amt::handle(ctx, coin_in_amount, coin_x_min_value)
    }

    pub fn swap_y(
        ctx: Context<SwapCoinY>,
        coin_in_amount: u64,
        coin_x_min_value: u64,
        ticket_number: u64
    ) -> Result<()> {
        swap_y::handle(ctx, coin_in_amount, coin_x_min_value, ticket_number)
    }

    pub fn init_staking_pool<'info>(
        ctx: Context<'_, '_, '_, 'info, InitStakingPool<'info>>,
    ) -> Result<()> {
        init_staking_pool::handle(ctx)
    }

    pub fn new_target_config(ctx: Context<NewTargetConfig>, target_amount: u64) -> Result<()> {
        new_target_config::handle(ctx, target_amount)
    }

    pub fn change_target_config(
        ctx: Context<ChangeTargetConfig>,
        target_amount: u64,
    ) -> Result<()> {
        change_target_config::handle(ctx, target_amount)
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

    pub fn staking_merge_tickets_non_zero(ctx: Context<StakingMergeTicketsNonZero>) -> Result<()> {
        staking_merge_tickets_nz::handle(ctx)
    }

    pub fn close_ticket(ctx: Context<CloseTicket>) -> Result<()> {
        close_ticket::handle(ctx)
    }

    pub fn withdraw_admin_fee(ctx: Context<WithdrawAdminFee>) -> Result<()> {
        withdraw_admin_fees::handle(ctx)
    }

    pub fn increase_vesting(ctx: Context<IncreaseVestingTime>, vesting_ts_increase: u64) -> Result<()> {
        increase_vesting_time::handle(ctx, vesting_ts_increase)
    }
}
