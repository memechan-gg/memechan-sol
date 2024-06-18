use solana_program::pubkey::Pubkey;

pub const MEME_TOKEN_DECIMALS: u64 = 1_000_000;
pub const WSOL_DECIMALS: u64 = 1_000_000_000;

pub const DEFAULT_PRICE_FACTOR_NUMERATOR: u64 = 80;
pub const DEFAULT_PRICE_FACTOR_DENOMINATOR: u64 = 100;

pub const DEFAULT_MAX_M_LP: u128 = 500_000_000_000_000;
pub const DEFAULT_MAX_M: u128 = 500_000_000_000_000;
pub const MAX_MEME_TOKENS: u128 = DEFAULT_MAX_M_LP + DEFAULT_MAX_M;

pub const DECIMALS_S: u128 = 1_000_000_000;

#[cfg(not(feature = "mainnet"))]
pub const LOCK_TIME: i64 = 60; // 1 minute
#[cfg(feature = "mainnet")]
pub const LOCK_TIME: i64 = 3600; // 1 hour

#[cfg(not(feature = "mainnet"))]
pub const DEFAULT_CLIFF: i64 = 180; // 3 minutes;
#[cfg(feature = "mainnet")]
pub const DEFAULT_CLIFF: i64 = 86_400; // 1 day;

#[cfg(not(feature = "mainnet"))]
pub const DEFAULT_LINEAR: i64 = 1800; // 0.5 hours;
#[cfg(feature = "mainnet")]
pub const DEFAULT_LINEAR: i64 = 604_800; // 7 days;

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
pub const ADMIN_KEY: Pubkey =
    solana_program::pubkey!("8SvkUtJZCyJwSQGkiszwcRcPv7c8pPSr8GVEppGNN7DV");
#[cfg(feature = "mainnet")]
pub const ADMIN_KEY: Pubkey =
    solana_program::pubkey!("KZbAoMgCcb2gDEn2Ucea86ux84y25y3ybbWQGQpd9D6");

#[cfg(not(feature = "mainnet"))]
pub const FEE_KEY: Pubkey = solana_program::pubkey!("feeLPZEfzJFwDR11cdMWE3nSa4nr7sPPM4u6tmDTw3Y");
#[cfg(feature = "mainnet")]
pub const FEE_KEY: Pubkey = solana_program::pubkey!("feeLPZEfzJFwDR11cdMWE3nSa4nr7sPPM4u6tmDTw3Y");
