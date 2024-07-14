use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct Vault {
    pub bump: u8,
    pub locked_nfts: u64,
}

pub fn create_vault(ctx: Context<CreateVault>) -> ProgramResult {
    let vault = &mut ctx.accounts.vault;
    vault.bump = *ctx.bumps.get("vault").unwrap();
    vault.locked_nfts = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct LockNFT<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub nft_account: Account<'info, TokenAccount>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
}

pub fn lock_nft(ctx: Context<LockNFT>, rent: u64) -> ProgramResult {
    let cpi_accounts = Transfer {
        from: ctx.accounts.nft_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, rent)?;

    let vault = &mut ctx.accounts.vault;
    vault.locked_nfts += 1;
    Ok(())
}
