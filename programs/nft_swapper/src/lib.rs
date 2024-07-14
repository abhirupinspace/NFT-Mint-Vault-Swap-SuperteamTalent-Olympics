use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

pub mod swap;

use swap::*;

declare_id!("2nqu4HEhvXDfKGQa83Urj2Wj7EsuUcKax6tkC5YbS3bR");

#[program]
pub mod nft_swapper {
    use super::*;

    pub fn swap_sol_for_nft(ctx: Context<SwapSolForNFT>, amount: u64) -> ProgramResult {
        swap::swap_sol_for_nft(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
