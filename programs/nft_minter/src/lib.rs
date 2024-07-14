use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};

pub mod mint;
pub mod vault;

use mint::*;
use vault::*;
declare_id!("2nqu4HEhvXDfKGQa83Urj2Wj7EsuUcKax6tkC5YbS3bR");

#[program]
pub mod nft_minter {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>, uri: String) -> ProgramResult {
        mint::mint_nft(ctx, uri)
    }

    pub fn create_vault(ctx: Context<CreateVault>) -> ProgramResult {
        vault::create_vault(ctx)
    }

    pub fn lock_nft(ctx: Context<LockNFT>, rent: u64) -> ProgramResult {
        vault::lock_nft(ctx, rent)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
