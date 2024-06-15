use crate::consts::{
    DEFAULT_MAX_M, DEFAULT_MAX_M_LP, DEFAULT_PRICE_FACTOR, FEE_KEY, MAX_MEME_TOKENS,
    MAX_TICKET_TOKENS, MEME_TOKEN_DECIMALS, SLERF_MINT,
};
use crate::err;
use crate::models::bound::{compute_alpha_abs, compute_beta, BoundPool, Config, Decimals};
use crate::models::fees::Fees;
use crate::models::fees::FEE;
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
            @ err::acc("meme mint authority must be the pool signer"),
        constraint = meme_mint.freeze_authority == COption::None
            @ err::acc("meme mint mustn't have a freeze authority"),
    )]
    pub meme_mint: Account<'info, Mint>,
    #[account(
        constraint = quote_vault.mint == quote_mint.key()
            @ err::acc("quote vault must be of ticket mint"),
        constraint = quote_vault.owner == pool_signer.key()
            @ err::acc("quote vault authority must match pool pda"),
        constraint = quote_vault.close_authority == COption::None
            @ err::acc("Quote vault must not have close authority"),
        constraint = quote_vault.delegate == COption::None
            @ err::acc("Quote vault must not have delegate"),
    )]
    pub quote_vault: Account<'info, TokenAccount>,
    #[account(
        constraint = quote_mint.key() == SLERF_MINT
            @ err::acc("Quote mint should be the SLERF mint")
    )]
    pub quote_mint: Account<'info, Mint>,
    #[account(
        constraint = fee_quote_vault.mint == quote_mint.key()
            @ err::acc("Fee quote vault must be of SLERF mint"),
        constraint = fee_quote_vault.owner == FEE_KEY
            @ err::acc("Fee quote vault authority must match admin"),
        constraint = fee_quote_vault.close_authority == COption::None
            @ err::acc("Fee quote vault must not have close authority"),
        constraint = fee_quote_vault.delegate == COption::None
            @ err::acc("Fee quote vault must not have delegate"),
    )]
    pub fee_quote_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = meme_vault.mint == meme_mint.key()
            @ err::acc("admin ticket vault must be of ticket mint"),
        constraint = meme_vault.owner == pool_signer.key()
            @ err::acc("Meme vault authority must match admin"),
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

pub fn handle(ctx: Context<NewPool>) -> Result<()> {
    let accs = ctx.accounts;

    if accs.meme_mint.supply != 0 {
        return Err(error!(err::acc("")));
    }

    let seeds = &[
        BoundPool::SIGNER_PDA_PREFIX,
        &accs.pool.key().to_bytes()[..],
        &[ctx.bumps.pool_signer],
    ];

    let signer_seeds = &[&seeds[..]];

    token::mint_to(
        accs.mint_meme_tokens().with_signer(signer_seeds),
        MAX_MEME_TOKENS * MEME_TOKEN_DECIMALS,
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
        fee_in_percent: FEE,
        fee_out_percent: FEE,
    };

    let mint_decimals = 10_u64.checked_pow(accs.quote_mint.decimals as u32).unwrap();
    let gamma_s = (accs.target_config.token_target_amount / mint_decimals) as u128;
    let gamma_m = DEFAULT_MAX_M;
    let omega_m = DEFAULT_MAX_M_LP;
    let price_factor = DEFAULT_PRICE_FACTOR;

    let (alpha_abs, decimals) = compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor)?;

    pool.config = Config {
        alpha_abs,
        beta: compute_beta(gamma_s, gamma_m, omega_m, price_factor, decimals)?,
        gamma_s: gamma_s as u64,
        gamma_m: gamma_m as u64,
        omega_m: omega_m as u64,
        price_factor,
        decimals: Decimals {
            alpha: decimals,
            beta: decimals,
            quote: mint_decimals,
        },
    };

    pool.meme_reserve.tokens = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;
    pool.meme_reserve.mint = accs.meme_mint.key();
    pool.meme_reserve.vault = accs.meme_vault.key();
    pool.locked = false;
    pool.creator_addr = accs.sender.key();

    Ok(())
}
