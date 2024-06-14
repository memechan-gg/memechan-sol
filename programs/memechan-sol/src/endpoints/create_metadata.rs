use crate::err;
use crate::models::bound::BoundPool;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::token;
use anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens;
use anchor_spl::token::SetAuthority;
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::state::Creator;
use mpl_token_metadata::state::DataV2;

#[derive(Accounts)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(constraint = sender.key() == pool.creator_addr)]
    pub pool: Account<'info, BoundPool>,
    #[account(
        mut,
        constraint = meme_mint.mint_authority == COption::Some(pool_signer.key())
            @ err::acc("meme mint authority must be the pool signer"),
        constraint = meme_mint.freeze_authority == COption::None
            @ err::acc("meme mint mustn't have a freeze authority"),
    )]
    pub meme_mint: Account<'info, Mint>,

    /// To store metaplex metadata. Created in the function scope
    /// CHECK: Is created via CPI call in the scope, checks made downstream
    #[account(mut)]
    pub meme_mpl_metadata: UncheckedAccount<'info>,

    /// CHECK: pool_pda
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// Program to create NFT metadata
    pub metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateMetadata<'info> {
    fn create_nft_with_metadata(
        &self,
        name: String,
        symbol: String,
        uri: String,
        bump: &[u8],
    ) -> Result<()> {
        let pool_key = self.pool.key();
        let seeds: &[&[u8]] = &[BoundPool::SIGNER_PDA_PREFIX, pool_key.as_ref(), bump];

        create_metadata_accounts_v3(
            self.create_metadata_account_v3().with_signer(&[&seeds]),
            DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: self.pool_signer.key(),
                    verified: true,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            false, // is_mutable
            true,  // update_authority_is_signer
            None,  // collection_details
        )?;

        token::set_authority(
            self.set_mint_authority(&self.meme_mint)
                .with_signer(&[seeds]),
            MintTokens,
            None,
        )
        .unwrap();

        Ok(())
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

    fn create_metadata_account_v3(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, CreateMetadataAccountsV3<'info>> {
        let cpi_accounts = CreateMetadataAccountsV3 {
            metadata: self.meme_mpl_metadata.to_account_info(),
            mint: self.meme_mint.to_account_info(),
            mint_authority: self.pool_signer.to_account_info(),
            payer: self.sender.to_account_info(),
            update_authority: self.pool_signer.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.metadata_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(
    ctx: Context<CreateMetadata>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let accs = ctx.accounts;

    let signer_bump_seed = *ctx.bumps.get("pool_signer").unwrap();
    accs.create_nft_with_metadata(name, symbol, uri, &[signer_bump_seed])?;

    Ok(())
}
