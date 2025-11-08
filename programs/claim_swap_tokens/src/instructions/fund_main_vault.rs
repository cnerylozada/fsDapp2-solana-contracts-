use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{constants::SWAP_TOKEN_TAG, errors::CustomError};

#[derive(Accounts)]
pub struct FundMainVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = recipient_token_account,
        seeds = [SWAP_TOKEN_TAG],
        bump
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn fund_main_vault(_context: Context<FundMainVault>, _amount: u64) -> Result<()> {
    let cpi_accounts = TransferChecked {
        mint: _context.accounts.token_mint.to_account_info(),
        from: _context.accounts.sender_token_account.to_account_info(),
        to: _context.accounts.recipient_token_account.to_account_info(),
        authority: _context.accounts.signer.to_account_info(),
    };
    let cpi_context = CpiContext::new(
        _context.accounts.token_program.to_account_info(),
        cpi_accounts,
    );

    let transfer_tx = transfer_checked(cpi_context, _amount, _context.accounts.token_mint.decimals);
    if transfer_tx.is_err() {
        return Err(CustomError::TransferError.into());
    }
    Ok(())
}
