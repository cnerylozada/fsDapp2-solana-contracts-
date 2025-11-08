use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::{ACCOUNT_DISCRIMINATOR, MAIN_VAULT_ACCOUNT_TAG, SWAP_TOKEN_TAG}, models::MainVaultAccount};

#[derive(Accounts)]
#[instruction(_title: String)]
pub struct CreateMainVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        space = ACCOUNT_DISCRIMINATOR + MainVaultAccount::INIT_SPACE,    
        payer = signer,
        seeds = [MAIN_VAULT_ACCOUNT_TAG, signer.key().as_ref(), _title.as_bytes()],
        bump
    )]
    pub main_vault_account: Account<'info, MainVaultAccount>,

    #[account(
        init,
        payer = signer,
        token::mint = token_mint,
        token::authority = token_vault,
        seeds = [SWAP_TOKEN_TAG, signer.key().as_ref(), _title.as_bytes()],
        bump
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn create_main_vault(_context: Context<CreateMainVault>, _title: String) -> Result<()> {
    let main_vault_account = &mut _context.accounts.main_vault_account;
    main_vault_account.title = _title;
    main_vault_account.token_mint = _context.accounts.token_mint.key();
    main_vault_account.bump = _context.bumps.main_vault_account;

    let now = Clock::get().unwrap().unix_timestamp;
    main_vault_account.created_at = now as u64;
    
    Ok(())
}

