use crate::consts::{POINTS_MINT, POINTS_PDA};
use crate::models::user_points::UserPoints;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct ReceivePoints<'info> {
    #[account(
        mut,
        seeds = [UserPoints::USER_POINTS_PREFIX, signer.key().as_ref()],
        bump
    )]
    user_points: Account<'info, UserPoints>,
    #[account(mut)]
    user_points_acc: Account<'info, TokenAccount>,
    #[account(mut, constraint = points_mint.key() == POINTS_MINT.key())]
    points_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = points_mint,
        token::authority = points_pda
    )]
    points_acc: Account<'info, TokenAccount>,
    #[account(mut)]
    signer: Signer<'info>,
    /// CHECK: pda signer
    #[account(seeds = [POINTS_PDA], bump)]
    points_pda: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

impl<'info> ReceivePoints<'info> {
    fn send_user_points(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.points_acc.to_account_info(),
            to: self.user_points_acc.to_account_info(),
            authority: self.points_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(ctx: Context<ReceivePoints>) -> Result<()> {
    let accs = ctx.accounts;

    let point_pda: &[&[u8]] = &[POINTS_PDA, &[ctx.bumps.points_pda]];

    let point_pda_seeds = &[&point_pda[..]];

    let points_to_receive = accs.user_points.points - accs.user_points.points_received;
    if points_to_receive > 0 {
        token::transfer(
            accs.send_user_points().with_signer(point_pda_seeds),
            points_to_receive,
        )
        .unwrap();
    }

    accs.user_points.points_received += points_to_receive;

    return Ok(());
}
