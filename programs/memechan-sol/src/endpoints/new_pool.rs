use crate::consts::{BP_FEE_KEY, DEFAULT_MAX_M, DEFAULT_MAX_M_LP, DEFAULT_PRICE_FACTOR_DENOMINATOR, DEFAULT_PRICE_FACTOR_NUMERATOR, MAX_AIRDROPPED_TOKENS, MAX_LINEAR, MAX_MEME_TOKENS, MAX_TH_FEE_BPS, MIN_LINEAR};
use crate::err;
use crate::err::AmmError;
use crate::models::bound::{compute_alpha_abs, compute_beta, BoundPool, Config, Decimals};
use crate::models::fees::FEE;
use crate::models::fees::{Fees, MEME_FEE};
use crate::models::target_config::TargetConfig;
use crate::models::Reserve;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct NewPool<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        init,
        payer = sender,
        space = BoundPool::space(),
        seeds = [BoundPool::POOL_PREFIX, meme_mint.key().as_ref(), quote_mint.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, BoundPool>,
    #[account(
        mut,
        constraint = meme_mint.mint_authority == COption::Some(pool_signer.key())
            @ err::acc("Meme mint authority must be the pool signer"),
        constraint = meme_mint.freeze_authority == COption::None
            @ err::acc("Meme mint mustn't have a freeze authority"),
    )]
    pub meme_mint: Account<'info, Mint>,
    #[account(
        constraint = quote_vault.mint == quote_mint.key()
            @ err::acc("Quote vault must be of ticket mint"),
        constraint = quote_vault.owner == pool_signer.key()
            @ err::acc("Quote vault authority must match the pool signer"),
        constraint = quote_vault.close_authority == COption::None
            @ err::acc("Quote vault must not have close authority"),
        constraint = quote_vault.delegate == COption::None
            @ err::acc("Quote vault must not have delegate"),
    )]
    pub quote_vault: Account<'info, TokenAccount>,
    pub quote_mint: Account<'info, Mint>,
    #[account(
        constraint = fee_quote_vault.mint == quote_mint.key()
            @ err::acc("Fee quote vault must be of quote mint"),
        constraint = fee_quote_vault.owner == BP_FEE_KEY
            @ err::acc("Fee quote vault authority must match fee key"),
        constraint = fee_quote_vault.close_authority == COption::None
            @ err::acc("Fee quote vault must not have close authority"),
        constraint = fee_quote_vault.delegate == COption::None
            @ err::acc("Fee quote vault must not have delegate"),
    )]
    pub fee_quote_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = meme_vault.mint == meme_mint.key()
            @ err::acc("Meme vault must be of meme mint"),
        constraint = meme_vault.owner == pool_signer.key()
            @ err::acc("Meme vault authority must match the pool signer"),
        constraint = meme_vault.close_authority == COption::None
            @ err::acc("Meme vault must not have close authority"),
        constraint = meme_vault.delegate == COption::None
            @ err::acc("Meme vault must not have delegate"),
    )]
    pub meme_vault: Account<'info, TokenAccount>,
    #[account(
        constraint = target_config.token_mint == quote_mint.key()
    )]
    pub target_config: Account<'info, TargetConfig>,
    /// CHECK: pool_pda
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> NewPool<'info> {
    fn mint_meme_tokens(&self) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_accounts = token::MintTo {
            mint: self.meme_mint.to_account_info(),
            to: self.meme_vault.to_account_info(),
            authority: self.pool_signer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(ctx: Context<NewPool>, airdropped_tokens: u64, vesting_period: i64, top_holder_fees_bps: u64) -> Result<()> {
    let accs = ctx.accounts;

    if accs.meme_mint.supply != 0 {
        return Err(error!(AmmError::NonZeroInitialMemeSupply));
    }

    if airdropped_tokens > MAX_AIRDROPPED_TOKENS {
        return Err(error!(AmmError::AirdroppedTokensOvercap));
    }

    if MIN_LINEAR > vesting_period || vesting_period > MAX_LINEAR {
        return Err(error!(AmmError::InvalidVestingPeriod));
    }

    if top_holder_fees_bps > MAX_TH_FEE_BPS {
        return Err(error!(AmmError::InvalidTHFeeBps))
    }

    let seeds = &[
        BoundPool::SIGNER_PDA_PREFIX,
        &accs.pool.key().to_bytes()[..],
        &[ctx.bumps.pool_signer],
    ];

    let signer_seeds = &[&seeds[..]];

    token::mint_to(
        accs.mint_meme_tokens().with_signer(signer_seeds),
        MAX_MEME_TOKENS as u64,
    )
    .unwrap();

    let pool = &mut accs.pool;
    pool.fee_vault_quote = accs.fee_quote_vault.key();
    pool.quote_reserve = Reserve {
        tokens: 0,
        mint: accs.quote_mint.key(),
        vault: accs.quote_vault.key(),
    };
    pool.fees = Fees {
        fee_meme_percent: MEME_FEE,
        fee_quote_percent: FEE,
    };

    let mint_decimals = 10_u128
        .checked_pow(accs.quote_mint.decimals as u32)
        .unwrap();
    let gamma_s = accs.target_config.token_target_amount as u128;
    let gamma_m = DEFAULT_MAX_M;
    let omega_m = DEFAULT_MAX_M_LP;
    let price_factor_num = DEFAULT_PRICE_FACTOR_NUMERATOR;
    let price_factor_denom = DEFAULT_PRICE_FACTOR_DENOMINATOR;

    let (alpha_abs, decimals) = compute_alpha_abs(
        gamma_s,
        mint_decimals,
        gamma_m,
        omega_m,
        price_factor_num,
        price_factor_denom,
    )?;

    pool.config = Config {
        alpha_abs,
        beta: compute_beta(
            gamma_s,
            mint_decimals,
            gamma_m,
            omega_m,
            price_factor_num,
            price_factor_denom,
            decimals,
        )?,
        gamma_s: gamma_s as u64,
        gamma_m: gamma_m as u64,
        omega_m: omega_m as u64,
        price_factor_num,
        price_factor_denom,
        decimals: Decimals {
            alpha: decimals,
            beta: decimals,
            quote: mint_decimals as u64,
        },
    };

    pool.meme_reserve.tokens = DEFAULT_MAX_M as u64;
    pool.meme_reserve.mint = accs.meme_mint.key();
    pool.meme_reserve.vault = accs.meme_vault.key();
    pool.locked = false;
    pool.creator_addr = accs.sender.key();
    pool.airdropped_tokens = airdropped_tokens;
    pool.vesting_period = vesting_period;

    Ok(())
}
