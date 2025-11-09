use anchor_lang::prelude::*;
use anchor_spl::{
    self,
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    constants::{ACCOUNT_DISCRIMINATOR, CLAIM_RECORD_TAG, MAX_TOKENS_CLAIMED, SWAP_TOKEN_TAG},
    errors::CustomError,
    models::ClaimRecord,
};

#[derive(Accounts)]
#[instruction(_admin: Pubkey)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = token_mint,
        token::authority = sender_token_account,
        seeds = [SWAP_TOKEN_TAG, token_mint.key().as_ref(), _admin.key().as_ref()],
        bump
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ACCOUNT_DISCRIMINATOR + ClaimRecord::INIT_SPACE,
        seeds = [CLAIM_RECORD_TAG, token_mint.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub claim_record: Account<'info, ClaimRecord>,

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

pub fn claim_tokens(_context: Context<ClaimTokens>, _admin: Pubkey, _amount: u64) -> Result<()> {
    let claim_record = &mut _context.accounts.claim_record;

    if claim_record.total_claimed == 0 {
        claim_record.user = _context.accounts.signer.key();
    }

    let factor: u64 = 10;
    let token_decimals = _context.accounts.token_mint.decimals;
    if claim_record.total_claimed + _amount
        > (MAX_TOKENS_CLAIMED * factor.pow(token_decimals as u32))
    {
        return Err(CustomError::ClaimLimitExceeded.into());
    }

    let cpi_acounts = TransferChecked {
        mint: _context.accounts.token_mint.to_account_info(),
        from: _context.accounts.sender_token_account.to_account_info(),
        to: _context.accounts.recipient_token_account.to_account_info(),
        authority: _context.accounts.sender_token_account.to_account_info(),
    };

    let token_mint_key = _context.accounts.token_mint.key();
    let admin_key = _admin.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        SWAP_TOKEN_TAG,
        token_mint_key.as_ref(),
        admin_key.as_ref(),
        &[_context.bumps.sender_token_account],
    ]];

    let cpi_context = CpiContext::new(
        _context.accounts.token_program.to_account_info(),
        cpi_acounts,
    )
    .with_signer(signer_seeds);

    let transfer_tx = transfer_checked(cpi_context, _amount, token_decimals);
    if transfer_tx.is_err() {
        return Err(CustomError::TransferError.into());
    }

    claim_record.total_claimed += _amount;

    Ok(())
}
