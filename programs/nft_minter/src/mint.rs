use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Token, TokenAccount};
use mpl_token_metadata::state::{Creator, Data, Metadata, TokenMetadataAccount};

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
}

pub fn mint_nft(ctx: Context<MintNFT>, uri: String) -> ProgramResult {
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, 1)?;

    let metadata = Metadata {
        name: "NFT".to_string(),
        symbol: "NFT".to_string(),
        uri,
        seller_fee_basis_points: 500,
        creators: Some(vec![Creator {
            address: *ctx.accounts.authority.key,
            verified: false,
            share: 100,
        }]),
        ..Default::default()
    };

    let metadata_account = TokenMetadataAccount::new(
        &ctx.accounts.mint.key(),
        ctx.accounts.authority.to_account_info(),
        metadata,
    );

    metadata_account.save()?;
    Ok(())
}
