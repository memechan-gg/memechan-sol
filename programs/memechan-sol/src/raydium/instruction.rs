//! Instruction types

#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
#![allow(deprecated)]
use super::models::{LastOrderDistance, RaydiumFees};
use anchor_lang::prelude::borsh::{self, BorshSerialize};
use anchor_lang::IdlBuild;
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct InitializeInstruction {
    /// nonce used to create valid program address
    pub nonce: u8,
    /// utc timestamps for pool open
    pub open_time: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct InitializeInstruction2 {
    /// nonce used to create valid program address
    pub nonce: u8,
    /// utc timestamps for pool open
    pub open_time: u64,
    /// init token pc amount
    pub init_pc_amount: u64,
    /// init token coin amount
    pub init_coin_amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct PreInitializeInstruction {
    /// nonce used to create valid program address
    pub nonce: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct MonitorStepInstruction {
    /// max value of plan/new/cancel orders
    pub plan_order_limit: u16,
    pub place_order_limit: u16,
    pub cancel_order_limit: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct DepositInstruction {
    /// Pool token amount to transfer. token_a and token_b amount are set by
    /// the current exchange rate and size of the pool
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct WithdrawInstruction {
    /// Pool token amount to transfer. token_a and token_b amount are set by
    /// the current exchange rate and size of the pool
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct SetParamsInstruction {
    pub param: u8,
    pub value: Option<u64>,
    pub new_pubkey: Option<Pubkey>,
    pub fees: Option<RaydiumFees>,
    pub last_order_distance: Option<LastOrderDistance>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct WithdrawSrmInstruction {
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct SwapInstructionBaseIn {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub minimum_amount_out: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct SwapInstructionBaseOut {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub max_amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub amount_out: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct SimulateInstruction {
    pub param: u8,
    pub swap_base_in_value: Option<SwapInstructionBaseIn>,
    pub swap_base_out_value: Option<SwapInstructionBaseOut>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct AdminCancelOrdersInstruction {
    pub limit: u16,
}

/// Update config acccount params
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize)]
pub(crate) struct ConfigArgs {
    pub param: u8,
    pub owner: Option<Pubkey>,
    pub create_pool_fee: Option<u64>,
}

/// Instructions supported by the AmmInfo program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize)]
pub(crate) enum AmmInstruction {
    ///   Initializes a new AmmInfo.
    ///
    ///   Not supported yet, please use `Initialize2` to new a AMM pool
    #[deprecated(note = "Not supported yet, please use `Initialize2` instead")]
    Initialize(InitializeInstruction),

    ///   Initializes a new AMM pool.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Associated Token program id
    ///   2. `[]` Sys program id
    ///   3. `[]` Rent program id
    ///   4. `[writable]` New AMM Account to create.
    ///   5. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   6. `[writable]` AMM open orders Account
    ///   7. `[writable]` AMM lp mint Account
    ///   8. `[]` AMM coin mint Account
    ///   9. `[]` AMM pc mint Account
    ///   10. `[writable]` AMM coin vault Account. Must be non zero, owned by $authority.
    ///   11. `[writable]` AMM pc vault Account. Must be non zero, owned by $authority.
    ///   12. `[writable]` AMM target orders Account. To store plan orders informations.
    ///   13. `[]` AMM config Account, derived from `find_program_address(&[&&AMM_CONFIG_SEED])`.
    ///   14. `[]` AMM create pool fee destination Account
    ///   15. `[]` Market program id
    ///   16. `[writable]` Market Account. Market program is the owner.
    ///   17. `[writable, singer]` User wallet Account
    ///   18. `[]` User token coin Account
    ///   19. '[]` User token pc Account
    ///   20. `[writable]` User destination lp token ATA Account
    Initialize2(InitializeInstruction2),

    ///   MonitorStep. To monitor place Amm order state machine turn around step by step.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Rent program id
    ///   2. `[]` Sys Clock id
    ///   3. `[writable]` AMM Account
    ///   4. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   5. `[writable]` AMM open orders Account
    ///   6. `[writable]` AMM target orders Account. To store plan orders infomations.
    ///   7. `[writable]` AMM coin vault Account. Must be non zero, owned by $authority.
    ///   8. `[writable]` AMM pc vault Account. Must be non zero, owned by $authority.
    ///   9. `[]` Market program id
    ///   10. `[writable]` Market Account. Market program is the owner.
    ///   11. `[writable]` Market coin vault Account
    ///   12. `[writable]` Market pc vault Account
    ///   13. '[]` Market vault signer Account
    ///   14. '[writable]` Market request queue Account
    ///   15. `[writable]` Market event queue Account
    ///   16. `[writable]` Market bids Account
    ///   17. `[writable]` Market asks Account
    ///   18. `[writable]` (optional) the (M)SRM account used for fee discounts
    ///   19. `[writable]` (optional) the referrer pc account used for settle back referrer
    MonitorStep(MonitorStepInstruction),

    ///   Deposit some tokens into the pool.  The output is a "pool" token representing ownership
    ///   into the pool. Inputs are converted to the current ratio.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[]` AMM open_orders Account
    ///   4. `[writable]` AMM target orders Account. To store plan orders infomations.
    ///   5. `[writable]` AMM lp mint Account. Owned by $authority.
    ///   6. `[writable]` AMM coin vault $authority can transfer amount,
    ///   7. `[writable]` AMM pc vault $authority can transfer amount,
    ///   8. `[]` Market Account. Market program is the owner.
    ///   9. `[writable]` User coin token Account to deposit into.
    ///   10. `[writable]` User pc token Account to deposit into.
    ///   11. `[writable]` User lp token. To deposit the generated tokens, user is the owner.
    ///   12. '[signer]` User wallet Account
    ///   13. `[]` Market event queue Account.
    Deposit(DepositInstruction),

    ///   Withdraw the vault tokens from the pool at the current ratio.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` AMM target orders Account
    ///   5. `[writable]` AMM lp mint Account. Owned by $authority.
    ///   6. `[writable]` AMM coin vault Account to withdraw FROM,
    ///   7. `[writable]` AMM pc vault Account to withdraw FROM,
    ///   8. `[]` Market program id
    ///   9. `[writable]` Market Account. Market program is the owner.
    ///   10. `[writable]` Market coin vault Account
    ///   11. `[writable]` Market pc vault Account
    ///   12. '[]` Market vault signer Account
    ///   13. `[writable]` User lp token Account.
    ///   14. `[writable]` User token coin Account. user Account to credit.
    ///   15. `[writable]` User token pc Account. user Account to credit.
    ///   16. `[singer]` User wallet Account
    ///   17. `[writable]` Market event queue Account
    ///   18. `[writable]` Market bids Account
    ///   19. `[writable]` Market asks Account
    Withdraw(WithdrawInstruction),

    ///   Migrate the associated market from Serum to OpenBook.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Sys program id
    ///   2. `[]` Rent program id
    ///   3. `[writable]` AMM Account
    ///   4. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   5. `[writable]` AMM open orders Account
    ///   6. `[writable]` AMM coin vault account owned by $authority,
    ///   7. `[writable]` AMM pc vault account owned by $authority,
    ///   8. `[writable]` AMM target orders Account
    ///   9. `[]` Market program id
    ///   10. `[writable]` Market Account. Market program is the owner.
    ///   11. `[writable]` Market bids Account
    ///   12. `[writable]` Market asks Account
    ///   13. `[writable]` Market event queue Account
    ///   14. `[writable]` Market coin vault Account
    ///   15. `[writable]` Market pc vault Account
    ///   16. '[]` Market vault signer Account
    ///   17. '[writable]` AMM new open orders Account
    ///   18. '[]` mew Market program id
    ///   19. '[]` new Market market Account
    ///   20. '[]` Admin Account
    MigrateToOpenBook,

    ///   Set AMM params
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account.
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` AMM target orders Account
    ///   5. `[writable]` AMM coin vault account owned by $authority,
    ///   6. `[writable]` AMM pc vault account owned by $authority,
    ///   7. `[]` Market program id
    ///   8. `[writable]` Market Account. Market program is the owner.
    ///   9. `[writable]` Market coin vault Account
    ///   10. `[writable]` Market pc vault Account
    ///   11. '[]` Market vault signer Account
    ///   12. `[writable]` Market event queue Account
    ///   13. `[writable]` Market bids Account
    ///   14. `[writable]` Market asks Account
    ///   15. `[singer]` Admin Account
    ///   16. `[]` (optional) New AMM open orders Account to replace old AMM open orders Account
    SetParams(SetParamsInstruction),

    ///   Withdraw Pnl from pool by protocol
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` AMM config Account, derived from `find_program_address(&[&&AMM_CONFIG_SEED])`.
    ///   3. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   4. `[writable]` AMM open orders Account
    ///   5. `[writable]` AMM coin vault account to withdraw FROM,
    ///   6. `[writable]` AMM pc vault account to withdraw FROM,
    ///   7. `[writable]` User coin token Account to withdraw to
    ///   8. `[writable]` User pc token Account to withdraw to
    ///   9. `[singer]` User wallet account
    ///   10. `[writable]` AMM target orders Account
    ///   11. `[]` Market program id
    ///   12. `[writable]` Market Account. Market program is the owner.
    ///   13. `[writable]` Market event queue Account
    ///   14. `[writable]` Market coin vault Account
    ///   15. `[writable]` Market pc vault Account
    ///   16. '[]` Market vault signer Account
    ///   17. `[]` (optional) the referrer pc account used for settle back referrer
    WithdrawPnl,

    ///   Withdraw (M)SRM from the (M)SRM Account used for fee discounts by admin
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` AMM Account.
    ///   2. `[singer]` Admin wallet Account
    ///   3. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   4. `[writable]` the (M)SRM Account withdraw from
    ///   5. `[writable]` the (M)SRM Account withdraw to
    WithdrawSrm(WithdrawSrmInstruction),

    /// Swap coin or pc from pool, base amount_in with a slippage of minimum_amount_out
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` (optional)AMM target orders Account, no longer used in the contract, recommended no need to add this Account.
    ///   5. `[writable]` AMM coin vault Account to swap FROM or To.
    ///   6. `[writable]` AMM pc vault Account to swap FROM or To.
    ///   7. `[]` Market program id
    ///   8. `[writable]` Market Account. Market program is the owner.
    ///   9. `[writable]` Market bids Account
    ///   10. `[writable]` Market asks Account
    ///   11. `[writable]` Market event queue Account
    ///   12. `[writable]` Market coin vault Account
    ///   13. `[writable]` Market pc vault Account
    ///   14. '[]` Market vault signer Account
    ///   15. `[writable]` User source token Account.
    ///   16. `[writable]` User destination token Account.
    ///   17. `[singer]` User wallet Account
    SwapBaseIn(SwapInstructionBaseIn),

    ///   Continue Initializes a new Amm pool because of compute units limit.
    ///   Not supported yet, please use `Initialize2` to new a Amm pool
    #[deprecated(note = "Not supported yet, please use `Initialize2` instead")]
    PreInitialize(PreInitializeInstruction),

    /// Swap coin or pc from pool, base amount_out with a slippage of max_amount_in
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` (optional)AMM target orders Account, no longer used in the contract, recommended no need to add this Account.
    ///   5. `[writable]` AMM coin vault Account to swap FROM or To.
    ///   6. `[writable]` AMM pc vault Account to swap FROM or To.
    ///   7. `[]` Market program id
    ///   8. `[writable]` Market Account. Market program is the owner.
    ///   9. `[writable]` Market bids Account
    ///   10. `[writable]` Market asks Account
    ///   11. `[writable]` Market event queue Account
    ///   12. `[writable]` Market coin vault Account
    ///   13. `[writable]` Market pc vault Account
    ///   14. '[]` Market vault signer Account
    ///   15. `[writable]` User source token Account.
    ///   16. `[writable]` User destination token Account.
    ///   17. `[singer]` User wallet Account
    SwapBaseOut(SwapInstructionBaseOut),

    SimulateInfo(SimulateInstruction),

    AdminCancelOrders(AdminCancelOrdersInstruction),

    /// Create amm config account by admin
    CreateConfigAccount,

    /// Update amm config account by admin
    UpdateConfigAccount(ConfigArgs),
}

impl IdlBuild for AmmInstruction {}
