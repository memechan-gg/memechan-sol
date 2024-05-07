use crate::consts::DEFAULT_MAX_M;
use crate::consts::DEFAULT_MAX_M_LP;
use crate::consts::DEFAULT_MAX_S;
use crate::consts::DEFAULT_PRICE_FACTOR;
use crate::consts::MAX_MEME_TOKENS;
use crate::consts::MAX_TICKET_TOKENS;
use crate::consts::MEME_TOKEN_DECIMALS;
use crate::err;
use crate::models::bound::compute_alpha_abs;
use crate::models::bound::compute_beta;
use crate::models::bound::BoundPool;
use crate::models::bound::Config;
use crate::models::fees::Fees;
use crate::models::fees::FEE;
use crate::models::Reserve;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens;
use anchor_spl::token::spl_token::native_mint;
use anchor_spl::token::{self, Mint, SetAuthority, Token, TokenAccount};

#[derive(Accounts)]
pub struct New<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        init,
        payer = sender,
        space = BoundPool::space()
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
        constraint = sol_vault.mint == sol_mint.key()
            @ err::acc("ticket vault must be of ticket mint"),
        constraint = sol_vault.owner == pool_signer.key()
            @ err::acc("ticket vault authority must match pool pda"),
    )]
    pub sol_vault: Account<'info, TokenAccount>,
    #[account(
        constraint = sol_mint.key() == native_mint::id()
            @ err::acc("sol mint should be native WSOL mint")
    )]
    pub sol_mint: Account<'info, Mint>,
    #[account(
        constraint = admin_sol_vault.mint == sol_mint.key()
            @ err::acc("admin sol vault must be of sol mint"),
        constraint = admin_sol_vault.owner == crate::admin::id()
            @ err::acc("admin sol vault authority must match admin"),
    )]
    pub admin_sol_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = launch_vault.mint == meme_mint.key()
            @ err::acc("admin ticket vault must be of ticket mint"),
        constraint = launch_vault.owner == pool_signer.key()
            @ err::acc("launch vault authority must match admin"),
    )]
    pub launch_vault: Account<'info, TokenAccount>,
    /// CHECK: pool_pda
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> New<'info> {
    fn mint_meme_tokens(&self) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_accounts = token::MintTo {
            mint: self.meme_mint.to_account_info(),
            to: self.launch_vault.to_account_info(),
            authority: self.pool_signer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn set_mint_authority(
        &self,
        mint: &Account<'info, Mint>,
    ) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            current_authority: self.pool_signer.to_account_info(),
            account_or_mint: mint.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(ctx: Context<New>) -> Result<()> {
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

    token::set_authority(
        accs.set_mint_authority(&accs.meme_mint)
            .with_signer(signer_seeds),
        MintTokens,
        None,
    )
    .unwrap();

    let pool = &mut accs.pool;
    pool.admin_vault_sol = accs.admin_sol_vault.key();
    pool.sol_reserve = Reserve {
        tokens: 0,
        mint: accs.sol_mint.key(),
        vault: accs.sol_vault.key(),
    };
    pool.fees = Fees {
        fee_in_percent: FEE,
        fee_out_percent: FEE,
    };

    let gamma_s = DEFAULT_MAX_S;
    let gamma_m = DEFAULT_MAX_M;
    let omega_m = DEFAULT_MAX_M_LP;
    let price_factor = DEFAULT_PRICE_FACTOR;

    pool.config = Config {
        alpha_abs: compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor).unwrap(),
        beta: compute_beta(gamma_s, gamma_m, omega_m, price_factor).unwrap(),
        gamma_s: gamma_s as u64,
        gamma_m: gamma_m as u64,
        omega_m: omega_m as u64,
        price_factor,
    };

    pool.meme_amt = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;
    pool.meme_mint = accs.meme_mint.key();
    pool.pool_meme_vault = accs.launch_vault.key();
    pool.locked = false;

    Ok(())
}
