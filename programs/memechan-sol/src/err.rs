use anchor_lang::prelude::*;
use std::fmt::Display;

#[error_code]
pub enum AmmError {
    /// Use this error via the [`acc`] function to provide more background
    /// about the issue.
    #[msg("Provided account breaks some constraints, see logs for more info")]
    InvalidAccountInput,
    /// Use this error via the [`arg`] function to provide more background
    /// about the issue.
    #[msg("One of the provided input arguments is invalid")]
    InvalidArg,
    #[msg(
        "Given amount of tokens to swap would result in less than minimum requested tokens to receive"
    )]
    SlippageExceeded,
    /// Use this error for program paths which should never be reached if the
    /// program logic works as intended.
    #[msg("There's a bug in the program, see logs for more info")]
    InvariantViolation,
    /// Use this error whenever trying to interact with a pool, but providing
    /// wrong token mints
    #[msg("Provided mints are not available on the pool")]
    InvalidTokenMints,
    MathOverflow,
    MulDivOverflow,
    DivideByZero,
    ZeroInAmt,
    ZeroMemeVault,
    InsufficientBalance,
    #[msg("Pool can't be interacted with until going into live phase")]
    PoolIsLocked,
    #[msg("Shouldn't provide zero tokens in")]
    NoZeroTokens,
    NoTokensToWithdraw,
    #[msg("Amount of tokens in ticket is lower than needed to swap")]
    NotEnoughTicketTokens,
    #[msg("Not enough time passed to unlock tokens bound to the ticket")]
    TicketTokensLocked,
    #[msg("Can't close ticket with non-zero bound token amount")]
    NonZeroAmountTicket,
    #[msg("Can't unstake the required amount of tokens")]
    NotEnoughTokensToRelease,
    BondingCurveMustBeNegativelySloped,
    BondingCurveInterceptMustBePositive,
    EGammaSAboveRelativeLimit,
    EScaleTooLow,
    InvalidAmmAccountOwner,
    ExpectedAccount,
    InvalidStatus,
    CantUnstakeBeforeCliff,
    NoFeesToAdd,
    #[msg("Staking should be fully initialized before it can be interacted with")]
    StakingIsNotActive,
    NonZeroInitialMemeSupply,
    AirdroppedTokensOvercap,
    InvalidVestingPeriod,
}

#[allow(dead_code)]
pub fn acc(msg: impl Display) -> AmmError {
    msg!("[InvalidAccountInput] {}", msg);

    AmmError::InvalidAccountInput
}

#[allow(dead_code)]
pub fn arg(msg: impl Display) -> AmmError {
    msg!("[InvalidArg] {}", msg);

    AmmError::InvalidArg
}
