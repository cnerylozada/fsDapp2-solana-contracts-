use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::SWAP_TOKEN_TAG;

#[derive(Accounts)]
pub struct CreateMainVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = admin,
        token::mint = token_mint,
        token::authority = token_vault,
        seeds = [SWAP_TOKEN_TAG, token_mint.key().as_ref(), admin.key().as_ref()],
        bump
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn create_main_vault(_context: Context<CreateMainVault>) -> Result<()> {
    Ok(())
}
