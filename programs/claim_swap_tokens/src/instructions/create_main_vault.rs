use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::SWAP_TOKEN_TAG;

#[derive(Accounts)]
pub struct CreateMainVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        token::mint = token_mint,
        token::authority = token_vault,
        seeds = [SWAP_TOKEN_TAG],
        bump
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn create_main_vault(_context: Context<CreateMainVault>, _title: String) -> Result<()> {
    Ok(())
}
