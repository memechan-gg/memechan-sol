pub mod instruction;
pub mod models;

pub use instruction::*;

pub use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;

use crate::consts::RAYDIUM_PROGRAM_ID;

#[derive(Clone)]
pub struct RaydiumAmm;

impl anchor_lang::Id for RaydiumAmm {
    fn id() -> Pubkey {
        RAYDIUM_PROGRAM_ID
    }
}

/// Creates an `Initialize2` instruction.
pub fn initialize2(
    program_id: &Pubkey, // raydium program
    // Params
    nonce: u8,
    open_time: u64,
    init_pc_amount: u64,
    init_coin_amount: u64,
    // Accounts
    spl_token_program: &Pubkey,
    ata_program: &Pubkey,
    sys_program: &Pubkey,
    rent_program: &Pubkey,
    amm: &Pubkey,
    amm_authority: &Pubkey,
    open_orders: &Pubkey,
    lp_mint: &Pubkey,
    coin_mint: &Pubkey,
    pc_mint: &Pubkey,
    coin_vault: &Pubkey,
    pc_vault: &Pubkey,
    target_orders: &Pubkey,
    amm_config: &Pubkey,
    fee_destination: &Pubkey,
    market_program_id: &Pubkey,
    market_account: &Pubkey,
    user: &Pubkey, // signer
    user_coin_wallet: &Pubkey,
    user_pc_wallet: &Pubkey,
    user_destination_lp_token_ata: &Pubkey,
) -> Instruction {
    let data = borsh::to_vec(&AmmInstruction::Initialize2(InitializeInstruction2 {
        nonce,
        open_time,
        init_pc_amount,
        init_coin_amount,
    }))
    .unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(*spl_token_program, false), // 0. `[]` Spl Token program id
        AccountMeta::new_readonly(*ata_program, false),       // 1. `[]` Associated Token program id
        AccountMeta::new_readonly(*sys_program, false),       // 2. `[]` Sys program id
        AccountMeta::new_readonly(*rent_program, false),      // 3. `[]` Rent program id
        AccountMeta::new(*amm, false), // 4. `[writable]` New AMM Account to create.
        AccountMeta::new_readonly(*amm_authority, false), // 5. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
        AccountMeta::new(*open_orders, false),            // 6. `[writable]` AMM open orders Account
        AccountMeta::new(*lp_mint, false),                // 7. `[writable]` AMM lp mint Account
        AccountMeta::new_readonly(*coin_mint, false),     // 8. `[]` AMM coin mint Account
        AccountMeta::new_readonly(*pc_mint, false),       // 9. `[]` AMM pc mint Account
        AccountMeta::new(*coin_vault, false), // 10. `[writable]` AMM coin vault Account. Must be non zero, owned by $authority.
        AccountMeta::new(*pc_vault, false), // 11. `[writable]` AMM pc vault Account. Must be non zero, owned by $authority.
        AccountMeta::new(*target_orders, false), // 12. `[writable]` AMM target orders Account. To store plan orders informations.
        AccountMeta::new_readonly(*amm_config, false), // 13. `[]` AMM config Account, derived from `find_program_address(&[&&AMM_CONFIG_SEED])`.
        AccountMeta::new(*fee_destination, false), // 14. `[]` AMM create pool fee destination Account
        AccountMeta::new_readonly(*market_program_id, false), // 15. `[]` Market program id
        AccountMeta::new(*market_account, false), // 16. `[writable]` Market Account. Market program is the owner.
        AccountMeta::new(*user, true),            // 17. `[writable, singer]` User wallet Account
        AccountMeta::new(*user_coin_wallet, false), // 18. `[]` User token coin Account
        AccountMeta::new(*user_pc_wallet, false), // 19. '[]` User token pc Account
        AccountMeta::new(*user_destination_lp_token_ata, false), // 20. `[writable]` User destination lp token ATA Account
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}

/// Creates an `Deposit` instruction.
pub fn deposit(
    program_id: &Pubkey, // raydium program
    // Params
    max_coin_amount: u64,
    max_pc_amount: u64,
    base_side: u64,
    // Accounts
    spl_token_program: &Pubkey,
    amm: &Pubkey,
    amm_authority: &Pubkey,
    open_orders: &Pubkey,
    target_orders: &Pubkey,
    lp_mint: &Pubkey,
    coin_vault: &Pubkey,
    pc_vault: &Pubkey,
    market_account: &Pubkey,
    user_coin_wallet: &Pubkey,
    user_pc_wallet: &Pubkey,
    user_lp_wallet: &Pubkey,
    user: &Pubkey, // user
    market_event_queue: &Pubkey,
) -> Instruction {
    let data = borsh::to_vec(&AmmInstruction::Deposit(DepositInstruction {
        max_coin_amount,
        max_pc_amount,
        base_side,
    }))
    .unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(*spl_token_program, false), // `[]` Spl Token program id
        AccountMeta::new(*amm, false),                        // `[writable]` AMM Account
        AccountMeta::new_readonly(*amm_authority, false), // `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
        AccountMeta::new_readonly(*open_orders, false),   // `[]` AMM open_orders Account
        AccountMeta::new(*target_orders, false), // `[writable]` AMM target orders Account. To store plan orders infomations.
        AccountMeta::new(*lp_mint, false), // `[writable]` AMM lp mint Account. Owned by $authority.
        AccountMeta::new(*coin_vault, false), // `[writable]` AMM coin vault $authority can transfer amount,
        AccountMeta::new(*pc_vault, false), // `[writable]` AMM pc vault $authority can transfer amount,
        AccountMeta::new_readonly(*market_account, false), // `[]` Market Account. Market program is the owner.
        AccountMeta::new(*user_coin_wallet, false), // `[writable]` User coin token Account to deposit into.
        AccountMeta::new(*user_pc_wallet, false), // `[writable]` User pc token Account to deposit into.
        AccountMeta::new(*user_lp_wallet, false), // `[writable]` User lp token. To deposit the generated tokens, user is the owner.
        AccountMeta::new(*user, true),            // '[signer]` User wallet Account
        AccountMeta::new_readonly(*market_event_queue, false), // `[]` Market event queue Account.
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}

/// Creates an `Withdraw` instruction.
pub fn withdraw(
    program_id: &Pubkey, // raydium program
    // Params
    amount: u64,
    // Accounts
    spl_token_program: &Pubkey,
    amm: &Pubkey,
    amm_authority: &Pubkey,
    open_orders: &Pubkey,
    target_orders: &Pubkey,
    lp_mint: &Pubkey,
    coin_vault: &Pubkey,
    pc_vault: &Pubkey,
    market_program_id: &Pubkey,
    market_account: &Pubkey,
    market_coin_vault: &Pubkey,
    market_pc_vault: &Pubkey,
    market_vault_signer: &Pubkey,
    user_lp_wallet: &Pubkey,
    user_coin_wallet: &Pubkey,
    user_pc_wallet: &Pubkey,
    user: &Pubkey, // user
    market_event_queue: &Pubkey,
    market_bids: &Pubkey,
    market_asks: &Pubkey,
) -> Instruction {
    let data = borsh::to_vec(&AmmInstruction::Withdraw(WithdrawInstruction { amount })).unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(*spl_token_program, false), // `[]` Spl Token program id
        AccountMeta::new(*amm, false),                        // `[writable]` AMM Account
        AccountMeta::new_readonly(*amm_authority, false), // `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
        AccountMeta::new(*open_orders, false),            // `[writable]` AMM open orders Account
        AccountMeta::new(*target_orders, false),          // `[writable]` AMM target orders Account
        AccountMeta::new(*lp_mint, false), // `[writable]` AMM lp mint Account. Owned by $authority.
        AccountMeta::new(*coin_vault, false), // `[writable]` AMM coin vault Account to withdraw FROM,
        AccountMeta::new(*pc_vault, false),   // `[writable]` AMM pc vault Account to withdraw FROM,
        AccountMeta::new_readonly(*market_program_id, false), // `[]` Market program id
        AccountMeta::new(*market_account, false), // `[writable]` Market Account. Market program is the owner.
        AccountMeta::new(*market_coin_vault, false), // `[writable]` Market coin vault Account
        AccountMeta::new(*market_pc_vault, false), // `[writable]` Market pc vault Account
        AccountMeta::new_readonly(*market_vault_signer, false), // '[]` Market vault signer Account
        AccountMeta::new(*user_lp_wallet, false), // `[writable]` User lp token Account.
        AccountMeta::new(*user_coin_wallet, false), // `[writable]` User token coin Account. user Account to credit.
        AccountMeta::new(*user_pc_wallet, false), // `[writable]` User token pc Account. user Account to credit.
        AccountMeta::new(*user, false),           // `[singer]` User wallet Account
        AccountMeta::new(*market_event_queue, false), // `[writable]` Market event queue Account
        AccountMeta::new(*market_bids, false),    // `[writable]` Market bids Account
        AccountMeta::new(*market_asks, false),    // `[writable]` Market asks Account
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}
