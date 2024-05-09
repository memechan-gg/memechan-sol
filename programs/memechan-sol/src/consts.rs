use solana_program::pubkey::Pubkey;

pub const MEME_TOKEN_DECIMALS: u64 = 1_000_000;
pub const WSOL_DECIMALS: u64 = 1_000_000_000;
pub const MAX_TICKET_TOKENS: u64 = 900_000_000;
pub const MAX_MEME_TOKENS: u64 = 1_125_000_000;

pub const DEFAULT_PRICE_FACTOR: u64 = 2;
pub const DEFAULT_MAX_M_LP: u128 = 200_000_000_000_000;
pub const DEFAULT_MAX_M: u128 = 900_000_000_000_000;
pub const DEFAULT_MAX_S: u128 = 300;

pub const DECIMALS_ALPHA: u128 = 1_000_000; // consider increase
pub const DECIMALS_BETA: u128 = 1_000_000; // consider increase
pub const DECIMALS_S: u128 = 1_000_000_000;

#[cfg(feature = "localnet")]
pub const RAYDIUM_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("EA68RLWrCRtwbiudgb25mHrFzuLBfCEVVhHEeuFF9sC6"); // Localnet

#[cfg(all(
    not(feature = "localnet"),
    not(feature = "mainnet"),
    feature = "devnet"
))]
pub const RAYDIUM_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8"); // Devnet
#[cfg(all(
    not(feature = "localnet"),
    not(feature = "devnet"),
    feature = "mainnet"
))]
pub const RAYDIUM_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"); // Devnet

#[cfg(feature = "localnet")]
pub const OPENBOOK_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("6jXACiuWwGjc2Hq7rzz7mLGZZnAjAgS6noCYtB31xx4u"); // Localnet

#[cfg(all(
    not(feature = "localnet"),
    not(feature = "mainnet"),
    feature = "devnet"
))]
pub const OPENBOOK_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj"); // Devnet
#[cfg(all(
    not(feature = "localnet"),
    not(feature = "devnet"),
    feature = "mainnet"
))]
pub const OPENBOOK_PROGRAM_ID: Pubkey =
    solana_program::pubkey!("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"); // Mainnet
