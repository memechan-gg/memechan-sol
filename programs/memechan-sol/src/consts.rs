use solana_program::pubkey::Pubkey;

pub const MEME_TOKEN_DECIMALS: u64 = 1_000_000;
pub const WSOL_DECIMALS: u64 = 1_000_000_000;
pub const MAX_TICKET_TOKENS: u64 = 900_000_000;
pub const MAX_MEME_TOKENS: u64 = 1_125_000_000;

pub const DEFAULT_PRICE_FACTOR: u64 = 2;
pub const DEFAULT_MAX_M_LP: u128 = 200_000_000_000_000;
pub const DEFAULT_MAX_M: u128 = 900_000_000_000_000;

pub const DECIMALS_S: u128 = 1_000_000_000;

// Raydium seeds
/// Suffix for amm authority seed
pub const AUTHORITY_AMM: &'static [u8] = b"amm authority";
/// Suffix for amm associated seed
pub const AMM_ASSOCIATED_SEED: &'static [u8] = b"amm_associated_seed";
/// Suffix for target associated seed
pub const TARGET_ASSOCIATED_SEED: &'static [u8] = b"target_associated_seed";
/// Suffix for amm open order associated seed
pub const OPEN_ORDER_ASSOCIATED_SEED: &'static [u8] = b"open_order_associated_seed";
/// Suffix for coin vault associated seed
pub const COIN_VAULT_ASSOCIATED_SEED: &'static [u8] = b"coin_vault_associated_seed";
/// Suffix for pc vault associated seed
pub const PC_VAULT_ASSOCIATED_SEED: &'static [u8] = b"pc_vault_associated_seed";
/// Suffix for lp mint associated seed
pub const LP_MINT_ASSOCIATED_SEED: &'static [u8] = b"lp_mint_associated_seed";
/// Amm config seed
pub const AMM_CONFIG_SEED: &'static [u8] = b"amm_config_account_seed";

#[cfg(not(feature = "mainnet"))]
pub const RAYDIUM_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8"); // Devnet
#[cfg(feature = "mainnet")]
pub const RAYDIUM_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"); // Mainnet

#[cfg(not(feature = "mainnet"))]
pub const OPENBOOK_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj"); // Devnet
#[cfg(feature = "mainnet")]
pub const OPENBOOK_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"); // Mainnet

#[cfg(not(feature = "mainnet"))]
pub const SLERF_MINT: Pubkey =
    solana_program::pubkey!("HX2pp5za2aBkrA5X5iTioZXcrpWb2q9DiaeWPW3qKMaw"); // Devnet - can use any
#[cfg(feature = "mainnet")]
pub const SLERF_MINT: Pubkey =
    solana_program::pubkey!("7BgBvyjrZX1YKz4oh9mjb8ZScatkkwb8DzFx7LoiVkM3"); // Mainnet

pub const ADMIN_KEY: Pubkey =
    solana_program::pubkey!("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX");
