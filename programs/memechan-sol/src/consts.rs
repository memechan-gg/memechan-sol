use solana_program::pubkey::Pubkey;

pub const MEME_TOKEN_DECIMALS: u64 = 1_000_000;
pub const WSOL_DECIMALS: u64 = 1_000_000_000;

pub const DEFAULT_PRICE_FACTOR_NUMERATOR: u64 = 1;
pub const DEFAULT_PRICE_FACTOR_DENOMINATOR: u64 = 1;

pub const DEFAULT_MAX_M_LP: u128 = 310_000_000_000_000;
pub const DEFAULT_MAX_M: u128 = 690_000_000_000_000;

pub const MAX_MEME_TOKENS: u128 = DEFAULT_MAX_M_LP + DEFAULT_MAX_M;

pub const DECIMALS_S: u128 = 1_000_000_000;

pub const MAX_AIRDROPPED_TOKENS: u64 = 100_000_000_000_000;

pub const POINTS_DECIMALS: u64 = 1_000_000_000;
pub const MAX_POINTS_AVAILABLE: u64 = 1_000_000_000 * POINTS_DECIMALS;
pub const BOOSTED_POINTS_AMOUNT: u64 = 300_000_000 * POINTS_DECIMALS;
pub const BOOSTED_SOL_AMOUNT: u64 = 100_000 * WSOL_DECIMALS;

#[cfg(feature = "localnet-testing")]
pub const LOCK_TIME: i64 = 4; // 4 seconds
#[cfg(feature = "mainnet-testing")]
pub const LOCK_TIME: i64 = 60; // 1 minute
#[cfg(feature = "mainnet")]
pub const LOCK_TIME: i64 = 3600; // 1 hour

#[cfg(feature = "localnet-testing")]
pub const DEFAULT_CLIFF: i64 = 5; // 5 seconds
#[cfg(feature = "mainnet-testing")]
pub const DEFAULT_CLIFF: i64 = 180; // 3 minutes
#[cfg(feature = "mainnet")]
pub const DEFAULT_CLIFF: i64 = 86_400; // 1 day

#[cfg(feature = "localnet-testing")]
pub const MIN_LINEAR: i64 = 10; // 10 seconds
#[cfg(feature = "mainnet-testing")]
pub const MIN_LINEAR: i64 = 600; // 10 minutes
#[cfg(feature = "mainnet")]
pub const MIN_LINEAR: i64 = 86_400; // 1 day
pub const MAX_LINEAR: i64 = 1_123_200; // 13 days

pub const INSTANT_TOKEN_PERCENTAGE_NUM: u64 = 10;
pub const INSTANT_TOKEN_PERCENTAGE_DENOM: u64 = 100;

pub const POINTS_PDA: &'static [u8; 10] = b"points_pda";

#[cfg(feature = "testing")]
pub const ADMIN_KEY: Pubkey =
    solana_program::pubkey!("8JvLLwD7oBvPfg3NL1dAL7GbQJuJznP4MhsYnfNkKjAR");
#[cfg(feature = "mainnet")]
pub const ADMIN_KEY: Pubkey =
    solana_program::pubkey!("KZbAoMgCcb2gDEn2Ucea86ux84y25y3ybbWQGQpd9D6");

#[cfg(feature = "testing")]
pub const SWAP_AUTH_KEY: Pubkey =
    solana_program::pubkey!("8JvLLwD7oBvPfg3NL1dAL7GbQJuJznP4MhsYnfNkKjAR");
#[cfg(feature = "mainnet")]
pub const SWAP_AUTH_KEY: Pubkey =
    solana_program::pubkey!("389y4YsTxFKpz2HxVHpvDk13FSXan48LZQtGv8pD4vQA");

pub const SWAP_FEE_KEY: Pubkey =
    solana_program::pubkey!("xqzvZzKFCjvPuRqkyg5rxA95avrvJxesZ41rCLfYwUM");
pub const LP_FEE_KEY: Pubkey =
    solana_program::pubkey!("HQ1wVLaBcnuoUozegyX7r45yn6ogHvQjdPNj53iweC5V");
pub const BP_FEE_KEY: Pubkey =
    solana_program::pubkey!("6YNJG9KDex3eNAmh1i64KUDbfKBiESkew3AWmnf6FiCy");

#[cfg(feature = "testing")]
pub const POINTS_MINT: Pubkey =
    solana_program::pubkey!("3evNjwM1tg4S9jCvg9vhA8JHcMtu4fVDHYteGGGzquJD");
#[cfg(feature = "mainnet")]
pub const POINTS_MINT: Pubkey =
    solana_program::pubkey!("ptsVM2dwpBVhu6uR3D1zzoRSjm1TC8gdmBEk8jpTP1P");
#[cfg(feature = "localnet-testing")]
pub const CHAN_MINT: Pubkey =
    solana_program::pubkey!("7wKaJ2mCthngM4RKsxCJYnyv9ZxWsw4TzEv8ZCkKaNnz");
#[cfg(feature = "mainnet-testing")]
pub const CHAN_MINT: Pubkey =
    solana_program::pubkey!("9pECN2xxLQo22bFYpsNr3T3eW1UdEDtSqPQopFrGv7n4");
#[cfg(feature = "mainnet")]
pub const CHAN_MINT: Pubkey =
    solana_program::pubkey!("ChanGGuDHboPswpTmKDfsTVGQL96VHhmvpwrE4UjWssd");
