use crate::consts::{MAX_TICKET_TOKENS, MEME_TOKEN_DECIMALS};
use crate::err::AmmError;
use crate::libraries::MulDiv;
use crate::models::bound::BoundPool;
use crate::models::fees::{LAUNCH_FEE, PRECISION};
use crate::models::staked_lp::MemeTicket;
use crate::models::staking::StakingPool;
use crate::{admin, vesting, RAYDIUM_PROGRAM_ID};
use crate::{err, raydium};
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
        has_one = pool_meme_vault,
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
    pub pool_meme_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = pool.sol_reserve.vault == pool_wsol_vault.key()
    )]
    pub pool_wsol_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub admin_vault_sol: Box<Account<'info, TokenAccount>>,
    #[account(init, payer = signer, space = MemeTicket::space())]
    pub meme_ticket: Box<Account<'info, MemeTicket>>,
    /// CHECK: bound-curve phase pda signer
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub bound_pool_signer_pda: AccountInfo<'info>,
    /// CHECK: live phase pda signer
    #[account(mut, seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_pool_signer_pda: AccountInfo<'info>,
    #[account(
        constraint = pool_meme_vault.mint == meme_mint.key()
    )]
    pub meme_mint: Box<Account<'info, Mint>>,
    #[account(
        constraint = sol_mint.key() == native_mint::id()
            @ err::acc("sol mint should be native WSOL mint")
    )]
    pub sol_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub raydium_lp_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub pool_lp_wallet: Box<Account<'info, TokenAccount>>,

    // signer and programs
    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,

    // raydium
    pub ata_program: AccountInfo<'info>,
    pub rent_program: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm: AccountInfo<'info>,
    pub raydium_amm_authority: AccountInfo<'info>,
    pub open_orders: AccountInfo<'info>,
    pub raydium_meme_vault: AccountInfo<'info>,
    pub raydium_wsol_vault: AccountInfo<'info>,
    pub target_orders: AccountInfo<'info>,
    pub amm_config: AccountInfo<'info>,
    pub fee_destination: AccountInfo<'info>,
    pub market_program_id: AccountInfo<'info>,
    pub market_account: AccountInfo<'info>,
    pub user_destination_lp_token_ata: AccountInfo<'info>,
    pub market_event_queue: AccountInfo<'info>,
}

impl<'info> GoLive<'info> {
    fn create_raydium_pool(
        &self,
        nonce: u8,
        open_time: u64,
        init_pc_amount: u64,
        init_coin_amount: u64,
        signer_seeds: &[&[&[u8]]; 1],
    ) -> Result<()> {
        let instruction = raydium::initialize2(
            &RAYDIUM_PROGRAM_ID,
            // Params
            nonce,
            open_time,
            init_pc_amount,
            init_coin_amount,
            // Accounts
            &self.token_program.key(),
            &self.ata_program.key(),
            &self.system_program.key(),
            &self.rent_program.key(),
            &self.raydium_amm.key(),
            &self.raydium_amm_authority.key(),
            &self.open_orders.key(),
            &self.raydium_lp_mint.key(),    // lp mint
            &self.meme_mint.key(),          // coin mint
            &self.sol_mint.key(),           // pc_mint
            &self.raydium_meme_vault.key(), // coin_vault
            &self.raydium_wsol_vault.key(), // pc_vault
            &self.target_orders.key(),
            &self.amm_config.key(),
            &self.fee_destination.key(),
            &self.market_program_id.key(),
            &self.market_account.key(),
            &self.signer.key(), // user/signer
            &self.pool_meme_vault.key(),
            &self.pool_wsol_vault.key(),
            &self.user_destination_lp_token_ata.key(),
        );
        solana_program::program::invoke_signed(
            &instruction,
            &[
                self.token_program.to_account_info().clone(),
                self.ata_program.to_account_info().clone(),
                self.system_program.to_account_info().clone(),
                self.rent_program.to_account_info().clone(),
                self.raydium_amm.to_account_info().clone(),
                self.raydium_amm_authority.to_account_info().clone(),
                self.open_orders.to_account_info().clone(),
                self.raydium_lp_mint.to_account_info().clone(),
                self.meme_mint.to_account_info().clone(),
                self.sol_mint.to_account_info().clone(),
                self.raydium_meme_vault.to_account_info().clone(),
                self.raydium_wsol_vault.to_account_info().clone(),
                self.target_orders.to_account_info().clone(),
                self.amm_config.to_account_info().clone(),
                self.fee_destination.to_account_info().clone(),
                self.market_program_id.to_account_info().clone(),
                self.market_account.to_account_info().clone(),
                self.signer.to_account_info().clone(),
                self.pool_meme_vault.to_account_info().clone(),
                self.pool_wsol_vault.to_account_info().clone(),
                self.user_destination_lp_token_ata.to_account_info().clone(),
            ],
            signer_seeds,
        )?;

        Ok(())
    }

    pub fn deposit_liquidity(
        &self,
        max_meme_amount: u64,
        max_wsol_amount: u64,
        signer_seeds: &[&[&[u8]]; 1],
    ) -> Result<()> {
        let instruction = raydium::deposit(
            &RAYDIUM_PROGRAM_ID,
            // Params
            max_meme_amount,
            max_wsol_amount,
            0, // base_side is meme token (i.e. 0)
            // Accounts
            &self.token_program.key(),
            &self.raydium_amm.key(),
            &self.raydium_amm_authority.key(),
            &self.open_orders.key(),
            &self.target_orders.key(),
            &self.raydium_lp_mint.key(),
            &self.raydium_meme_vault.key(),
            &self.pool_wsol_vault.key(),
            &self.market_account.key(),
            &self.pool_meme_vault.key(),
            &self.pool_wsol_vault.key(),
            &self.pool_lp_wallet.key(),
            &self.signer.key(),
            &self.market_event_queue.key(),
        );

        solana_program::program::invoke_signed(
            &instruction,
            &[
                self.token_program.to_account_info().clone(),
                self.raydium_amm.to_account_info().clone(),
                self.raydium_amm_authority.to_account_info().clone(),
                self.open_orders.to_account_info().clone(),
                self.target_orders.to_account_info().clone(),
                self.raydium_lp_mint.to_account_info().clone(),
                self.raydium_meme_vault.to_account_info().clone(),
                self.pool_wsol_vault.to_account_info().clone(),
                self.market_account.to_account_info().clone(),
                self.pool_meme_vault.to_account_info().clone(),
                self.pool_wsol_vault.to_account_info().clone(),
                self.pool_lp_wallet.to_account_info().clone(),
                self.signer.to_account_info().clone(),
                self.market_event_queue.to_account_info().clone(),
            ],
            signer_seeds,
        )?;

        Ok(())
    }

    // pub fn deposit_liquidity_ctx(&self) -> CpiContext<'_, '_, '_, 'info, DepositLiquidity<'info>> {
    //     let cpi_program = self.aldrin_amm_program.to_account_info();
    //     let cpi_accounts = DepositLiquidity {
    //         user: self.staking_pool_signer_pda.to_account_info(),
    //         pool: self.aldrin_pool_acc.to_account_info(),
    //         pool_signer_pda: self.aldrin_pool_signer.to_account_info(),
    //         lp_mint: self.aldrin_lp_mint.to_account_info(),
    //         lp_token_wallet: self.lp_token_wallet.to_account_info(),
    //         token_program: self.token_program.to_account_info(),
    //     };
    //     CpiContext::new(cpi_program, cpi_accounts)
    // }

    pub fn set_lp_wallet_authority(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            current_authority: self.bound_pool_signer_pda.to_account_info(),
            account_or_mint: self.pool_meme_vault.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn set_vault_authority_ctx(
        &self,
        account: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            current_authority: self.bound_pool_signer_pda.to_account_info(),
            account_or_mint: account,
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
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

pub fn handle<'info>(ctx: Context<'_, '_, '_, 'info, GoLive<'info>>, nonce: u8) -> Result<()> {
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
    let meme_supply = accs.pool_meme_vault.amount;
    let meme_supply_80 = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;

    let amm_meme_balance = meme_supply.checked_sub(meme_supply_80).unwrap();

    // 4. Transfer pool_wsol_vault
    msg!("4");

    token::set_authority(
        accs.set_vault_authority_ctx(accs.pool_wsol_vault.to_account_info())
            .with_signer(bp_signer_seeds),
        AuthorityType::AccountOwner,
        Some(accs.staking_pool_signer_pda.key()),
    )
    .unwrap();
    token::set_authority(
        accs.set_vault_authority_ctx(accs.pool_meme_vault.to_account_info())
            .with_signer(bp_signer_seeds),
        AuthorityType::AccountOwner,
        Some(accs.staking_pool_signer_pda.key()),
    )
    .unwrap();

    // 5. Setup new staking account
    msg!("5");
    let staking = &mut accs.staking;

    staking.meme_vault = accs.pool_meme_vault.key();
    staking.meme_mint = accs.meme_mint.key();
    staking.wsol_vault = accs.pool_wsol_vault.key();
    staking.stakes_total = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;
    staking.vesting_config = vesting::default_config();
    staking.fees_x_total = 0;
    staking.fees_y_total = 0;
    staking.pool = accs.pool.key();

    // 6. Initialize pool & Add liquidity to the pool
    msg!("6");
    accs.create_raydium_pool(
        nonce,
        accs.clock.unix_timestamp as u64, // open time
        sol_supply - live_fee_amt,        // init_pc_amount
        amm_meme_balance,                 // init_coin_amount
        staking_signer_seeds,
    );

    // No need to add liquidity as already added above in init instruciton
    // accs.deposit_liquidity(
    //     amm_meme_balance, //max_meme_amount
    //     sol_supply - live_fee_amt, // max_wsol_amount
    //     staking_signer_seeds,,
    // );

    // 7. Set lp authority to the system program, i.e. burn LPs
    msg!("7");
    token::set_authority(
        accs.set_lp_wallet_authority().with_signer(bp_signer_seeds),
        AuthorityType::AccountOwner,
        Some(accs.system_program.key()),
    )
    .unwrap();
    msg!("8");

    Ok(())
}
