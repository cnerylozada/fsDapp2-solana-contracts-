use anchor_lang::prelude::*;
use anchor_spl::{
    self,
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{constants::SWAP_TOKEN_TAG, errors::CustomError};

#[derive(Accounts)]
#[instruction(_title: String)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = sender_token_account,
        seeds = [SWAP_TOKEN_TAG, signer.key().as_ref(), _title.as_bytes()],
        bump
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn claim_tokens(_context: Context<ClaimTokens>, _title: String, _amount: u64) -> Result<()> {
    let cpi_acounts = TransferChecked {
        mint: _context.accounts.token_mint.to_account_info(),
        from: _context.accounts.sender_token_account.to_account_info(),
        to: _context.accounts.recipient_token_account.to_account_info(),
        authority: _context.accounts.sender_token_account.to_account_info(),
    };

    let signer_key = _context.accounts.signer.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        SWAP_TOKEN_TAG,
        signer_key.as_ref(),
        _title.as_bytes(),
        &[_context.bumps.sender_token_account],
    ]];

    let cpi_context = CpiContext::new(
        _context.accounts.token_program.to_account_info(),
        cpi_acounts,
    )
    .with_signer(signer_seeds);

    let transfer_tx = transfer_checked(cpi_context, _amount, _context.accounts.token_mint.decimals);
    if transfer_tx.is_err() {
        return Err(CustomError::TransferError.into());
    }

    Ok(())
}
