use anchor_lang::prelude::*;
use anchor_spl::{
    self,
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::constants::SWAP_TOKEN_TAG;

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = sender_token_account,
        seeds = [SWAP_TOKEN_TAG],
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
    // let signer_seeds: &[&[&[u8]]] = &[&[b"token", &[_context.bumps.tokenx_x_vault]]];

    // let transfer_accounts = TransferChecked {
    //     mint: _context.accounts.token_mint_x.to_account_info(),
    //     from: _context.accounts.tokenx_x_vault.to_account_info(),
    //     to: _context
    //         .accounts
    //         .recipient_token_x_account
    //         .to_account_info(),
    //     authority: _context.accounts.tokenx_x_vault.to_account_info(),
    // };
    // let cpi_context = CpiContext::new(
    //     _context.accounts.token_program.to_account_info(),
    //     transfer_accounts,
    // )
    // .with_signer(signer_seeds);

    // let transfer_tx = transfer_checked(
    //     cpi_context,
    //     _amount,
    //     _context.accounts.token_mint_x.decimals,
    // );

    // if transfer_tx.is_err() {
    //     return Err(TransferError.into());
    // }

    // Ok(())
    Ok(())
}
