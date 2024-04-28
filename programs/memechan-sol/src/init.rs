use crate::err;
use crate::err::AmmError;
use crate::fees::{LAUNCH_FEE, PRECISION};
use crate::libraries::MulDiv;
use crate::staked_lp::MemeTicket;
use crate::staking::StakingPool;
use crate::{admin, vesting, BoundPool, MAX_TICKET_TOKENS, MEME_TOKEN_DECIMALS};
use amm;
use amm::cpi::accounts::{CreatePool, DepositLiquidity};
use amm::cpi::{create_pool, deposit_liquidity};
use amm::models::{TokenAmount, TokenLimit};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::spl_token::instruction::AuthorityType;
use anchor_spl::token::spl_token::native_mint;
use anchor_spl::token::{Mint, SetAuthority, Token, TokenAccount, Transfer};

//const ADMIN_ADDR: address = @0xfff; // TODO
//
const SOL_THRESHOLD: u64 = 300;

// const A: u128 = 400_000;
// const GAMMA: u128 = 145_000_000_000_000;
//
// const ALLOWED_EXTRA_PROFIT: u128 = 2000000000000; // 18 decimals
// const ADJUSTMENT_STEP: u128 = 146000000000000; // 18 decimals
// const MA_TIME: u128 = 600_000; // 10 minutes
//
// const MID_FEE: u128 = 260_000_000_000_000_000; // (0.26%) swap fee when the pool is balanced
// const OUT_FEE: u128 = 450_000_000_000_000_000; // (0.45%) swap fee when the pool is out balance
// const GAMMA_FEE: u128 = 200_000_000_000_000; //  (0.0002%) speed rate that fee increases mid_fee => out_fee
//
// /// The amount of Mist per Sui token based on the fact that mist is
// /// 10^-9 of a Sui token
// const MIST_PER_SUI: u64 = 1_000_000_000;
//

#[derive(Accounts)]
pub struct GoLive<'info> {
    // go_live instruction accounts
    #[account(
        mut,
        close = signer,
        has_one = launch_token_vault,
        has_one = admin_vault_sol
    )]
    pub pool: Box<Account<'info, BoundPool>>,
    #[account(
        init,
        payer = signer,
        space = StakingPool::space()
    )]
    pub staking: Account<'info, StakingPool>,
    #[account(mut)]
    pub launch_token_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = pool.sol_reserve.vault == pool_wsol_vault.key()
    )]
    pub pool_wsol_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub admin_vault_sol: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = staking_meme_vault.mint == launch_token_vault.mint
    )]
    pub staking_meme_vault: Box<Account<'info, TokenAccount>>,
    #[account(init, payer = signer, space = MemeTicket::space())]
    pub meme_ticket: Box<Account<'info, MemeTicket>>,
    /// CHECK: bound-curve phase pda signer
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub bound_pool_signer_pda: AccountInfo<'info>,
    /// CHECK: live phase pda signer
    #[account(mut, seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_pool_signer_pda: AccountInfo<'info>,
    #[account(
        constraint = launch_token_vault.mint == meme_mint.key()
    )]
    pub meme_mint: Box<Account<'info, Mint>>,
    #[account(
        constraint = sol_mint.key() == native_mint::id()
            @ err::acc("sol mint should be native WSOL mint")
    )]
    pub sol_mint: Box<Account<'info, Mint>>,

    // aldrin pool creation
    /// CHECK: checked by AMM
    #[account(mut)]
    pub aldrin_pool_acc: Signer<'info>,
    /// CHECK: checked by AMM
    pub aldrin_pool_signer: AccountInfo<'info>,
    /// CHECK: checked by AMM
    pub aldrin_program_toll: AccountInfo<'info>,
    pub aldrin_program_toll_wallet: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub aldrin_lp_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub lp_token_wallet: Box<Account<'info, TokenAccount>>,

    // signer and programs
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: comparing with dep ID
    #[account(
        constraint = aldrin_amm_program.key() == amm::id()
    )]
    pub aldrin_amm_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> GoLive<'info> {
    pub fn create_pool_ctx(&self) -> CpiContext<'_, '_, '_, 'info, CreatePool<'info>> {
        let cpi_program = self.aldrin_amm_program.to_account_info();
        let cpi_accounts = CreatePool {
            admin: self.staking_pool_signer_pda.to_account_info(),
            pool: self.aldrin_pool_acc.to_account_info(),
            pool_signer: self.aldrin_pool_signer.to_account_info(),
            program_toll: self.aldrin_program_toll.to_account_info(),
            program_toll_wallet: self.aldrin_program_toll_wallet.to_account_info(),
            lp_mint: self.aldrin_lp_mint.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn deposit_liquidity_ctx(&self) -> CpiContext<'_, '_, '_, 'info, DepositLiquidity<'info>> {
        let cpi_program = self.aldrin_amm_program.to_account_info();
        let cpi_accounts = DepositLiquidity {
            user: self.bound_pool_signer_pda.to_account_info(),
            pool: self.aldrin_pool_acc.to_account_info(),
            pool_signer_pda: self.aldrin_pool_signer.to_account_info(),
            lp_mint: self.aldrin_lp_mint.to_account_info(),
            lp_token_wallet: self.lp_token_wallet.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn set_lp_wallet_authority(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            current_authority: self.bound_pool_signer_pda.to_account_info(),
            account_or_mint: self.lp_token_wallet.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> GoLive<'info> {
    fn send_admin_fee_sol(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.pool_wsol_vault.to_account_info(),
            to: self.admin_vault_sol.to_account_info(),
            authority: self.bound_pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn go_live_handler<'info>(ctx: Context<'_, '_, '_, 'info, GoLive<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    let bp_seeds = &[
        BoundPool::SIGNER_PDA_PREFIX,
        &accs.pool.key().to_bytes()[..],
        &[ctx.bumps.bound_pool_signer_pda],
    ];

    let bp_signer_seeds = &[&bp_seeds[..]];

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_pool_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    // 0. Create admin ticket + withdraw sol fees
    msg!("0");
    let meme_ticket = &mut accs.meme_ticket;

    meme_ticket.setup(accs.pool.key(), admin::id(), accs.pool.admin_fees_meme);

    if accs.pool.admin_fees_sol != 0 {
        token::transfer(
            accs.send_admin_fee_sol().with_signer(bp_signer_seeds),
            accs.pool.admin_fees_sol,
        )
        .unwrap();
    };

    // 1. Verify if we reached the threshold of SUI amount raised
    msg!("1");
    accs.pool_wsol_vault.reload().unwrap();
    let sol_supply = accs.pool_wsol_vault.amount;
    if sol_supply != SOL_THRESHOLD * 10_u64.checked_pow(native_mint::DECIMALS as u32).unwrap() {
        return Err(error!(AmmError::InvariantViolation));
    }

    // 2. Collect live fees
    msg!("2");
    let live_fee_amt = sol_supply.mul_div_ceil(LAUNCH_FEE, PRECISION).unwrap();
    token::transfer(
        accs.send_admin_fee_sol().with_signer(bp_signer_seeds),
        live_fee_amt,
    )
    .unwrap();

    // 3. Split MEME balance amounts into 80/20
    msg!("3");
    let meme_supply = accs.launch_token_vault.amount;
    let meme_supply_80 = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;

    let amm_meme_balance = meme_supply.checked_sub(meme_supply_80).unwrap();

    // 4. Transfer
    msg!("4");

    // 5. Setup new staking account
    msg!("5");
    let staking = &mut accs.staking;

    staking.meme_vault = accs.staking_meme_vault.key();
    staking.wsol_vault = accs.pool_wsol_vault.key();
    staking.stakes_total = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;
    staking.vesting_config = vesting::default_config();
    staking.fees_x_total = 0;
    staking.fees_y_total = 0;
    msg!("6");
    // 6. Initialize pool

    create_pool(
        accs.create_pool_ctx()
            .with_signer(staking_signer_seeds)
            .with_remaining_accounts(ctx.remaining_accounts.to_vec()),
        0,
    )
    .unwrap();

    // 7. Add liquidity to the pool and burn LPs
    msg!("7");
    let max_amt_tokens = vec![
        TokenLimit {
            mint: accs.meme_mint.key(),
            tokens: TokenAmount {
                amount: amm_meme_balance,
            },
        },
        TokenLimit {
            mint: accs.sol_mint.key(),
            tokens: TokenAmount {
                amount: sol_supply - live_fee_amt,
            },
        },
    ];
    deposit_liquidity(
        accs.deposit_liquidity_ctx()
            .with_remaining_accounts(vec![
                ctx.remaining_accounts[0].to_account_info(),
                accs.launch_token_vault.to_account_info(),
                ctx.remaining_accounts[1].to_account_info(),
                accs.pool_wsol_vault.to_account_info(),
            ])
            .with_signer(bp_signer_seeds),
        max_amt_tokens,
    )
    .unwrap();
    msg!("7.1");
    token::set_authority(
        accs.set_lp_wallet_authority().with_signer(bp_signer_seeds),
        AuthorityType::AccountOwner,
        Some(accs.system_program.key()),
    )
    .unwrap();
    msg!("8");

    Ok(())
}
