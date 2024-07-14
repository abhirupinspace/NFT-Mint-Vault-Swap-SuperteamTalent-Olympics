use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct SwapSolForNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub protocol_account: Account<'info, TokenAccount>,
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

pub fn swap_sol_for_nft(ctx: Context<SwapSolForNFT>, amount: u64) -> ProgramResult {
    // Transfer SOL to protocol account
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        ctx.accounts.payer.key,
        ctx.accounts.protocol_account.to_account_info().key,
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.protocol_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Transfer NFT from vault to receiver
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.receiver.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, 1)?;

    Ok(())
}
