use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::SWAP_TOKEN_TAG;

#[derive(Accounts)]
#[instruction(_id: String)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        token::mint = token_mint,
        token::authority = token_vault,
        seeds = [SWAP_TOKEN_TAG, token_mint.key().as_ref(), signer.key().as_ref(), _id.as_bytes()],
        bump
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn create_vault(_context: Context<CreateVault>, _id: String) -> Result<()> {
    Ok(())
}
