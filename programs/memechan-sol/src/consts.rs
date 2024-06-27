use solana_program::pubkey::Pubkey;

pub const MEME_TOKEN_DECIMALS: u64 = 1_000_000;
pub const WSOL_DECIMALS: u64 = 1_000_000_000;

pub const DEFAULT_PRICE_FACTOR_NUMERATOR: u64 = 80;
pub const DEFAULT_PRICE_FACTOR_DENOMINATOR: u64 = 100;

pub const DEFAULT_MAX_M_LP: u128 = 500_000_000_000_000;
pub const DEFAULT_MAX_M: u128 = 500_000_000_000_000;

pub const MAX_MEME_TOKENS: u128 = DEFAULT_MAX_M_LP + DEFAULT_MAX_M;

pub const DECIMALS_S: u128 = 1_000_000_000;

pub const MAX_AIRDROPPED_TOKENS: u64 = 300_000_000_000_000;

#[cfg(not(feature = "mainnet"))]
pub const LOCK_TIME: i64 = 60; // 1 minute
#[cfg(feature = "mainnet")]
pub const LOCK_TIME: i64 = 180; // 1 hour

#[cfg(not(feature = "mainnet"))]
pub const DEFAULT_CLIFF: i64 = 180; // 3 minutes;
#[cfg(feature = "mainnet")]
pub const DEFAULT_CLIFF: i64 = 600; // 1 day;

#[cfg(not(feature = "mainnet"))]
pub const DEFAULT_LINEAR: i64 = 1800; // 0.5 hours;
#[cfg(feature = "mainnet")]
pub const DEFAULT_LINEAR: i64 = 3600; // 7 days;

pub const INSTANT_TOKEN_PERCENTAGE_NUM: u64 = 10;
pub const INSTANT_TOKEN_PERCENTAGE_DENOM: u64 = 100;

#[cfg(not(feature = "mainnet"))]
pub const ADMIN_KEY: Pubkey =
    solana_program::pubkey!("8SvkUtJZCyJwSQGkiszwcRcPv7c8pPSr8GVEppGNN7DV");
#[cfg(feature = "mainnet")]
pub const ADMIN_KEY: Pubkey =
    solana_program::pubkey!("8SvkUtJZCyJwSQGkiszwcRcPv7c8pPSr8GVEppGNN7DV");

#[cfg(not(feature = "mainnet"))]
pub const SWAP_AUTH_KEY: Pubkey =
    solana_program::pubkey!("8SvkUtJZCyJwSQGkiszwcRcPv7c8pPSr8GVEppGNN7DV");
#[cfg(feature = "mainnet")]
pub const SWAP_AUTH_KEY: Pubkey =
    solana_program::pubkey!("8SvkUtJZCyJwSQGkiszwcRcPv7c8pPSr8GVEppGNN7DV");

pub const FEE_KEY: Pubkey = solana_program::pubkey!("feeLPZEfzJFwDR11cdMWE3nSa4nr7sPPM4u6tmDTw3Y");

#[cfg(not(feature = "mainnet"))]
pub const CHAN_MINT: Pubkey =
    solana_program::pubkey!("HX2pp5za2aBkrA5X5iTioZXcrpWb2q9DiaeWPW3qKMaw");
#[cfg(feature = "mainnet")]
pub const CHAN_MINT: Pubkey =
    solana_program::pubkey!("59uVEJ3baADZ7Pg79MtV1erDrqGXYQK7qqJg3JwjGgm3");
