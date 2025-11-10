use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    constants::{OFFER_TAG, SWAP_TOKEN_TAG},
    errors::CustomError,
    instructions::shared::transfer_tokens,
    models::Offer,
};

#[derive(Accounts)]
#[instruction(_id: String, _maker: Pubkey)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub sender_token_account_b: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [OFFER_TAG, _maker.key().as_ref(), _id.as_bytes()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,

    pub token_program: Interface<'info, TokenInterface>,

    #[account(mut)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        token::mint = token_mint_a,
        token::authority = sender_token_account_a,
        seeds = [SWAP_TOKEN_TAG, token_mint_a.key().as_ref(), _maker.key().as_ref(), _id.as_bytes()],
        bump
    )]
    pub sender_token_account_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint_a,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub recipient_token_account_a: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn send_wanted_tokens_to_maker(_context: &Context<TakeOffer>, _id: &String) -> Result<()> {
    // let cpi_accounts = TransferChecked {
    //     mint: _context.accounts.token_mint_b.to_account_info(),
    //     from: _context.accounts.sender_token_account_b.to_account_info(),
    //     to: _context
    //         .accounts
    //         .recipient_token_account_b
    //         .to_account_info(),
    //     authority: _context.accounts.signer.to_account_info(),
    // };

    // let cpi_context = CpiContext::new(
    //     _context.accounts.token_program.to_account_info(),
    //     cpi_accounts,
    // );

    let token_wanted_amount = _context.accounts.offer.token_wanted_amount;
    // let transfer_tx = transfer_checked(
    //     cpi_context,
    //     token_wanted_amount,
    //     _context.accounts.token_mint_b.decimals,
    // );

    // if transfer_tx.is_err() {
    //     return Err(CustomError::TransferError.into());
    // }

    // Ok(())

    transfer_tokens(
        &_context.accounts.token_mint_b,
        &_context.accounts.sender_token_account_b,
        &_context.accounts.recipient_token_account_b,
        &_context.accounts.signer,
        &_context.accounts.token_program,
        token_wanted_amount,
    )
}

pub fn withdraw_offered_tokens(_context: Context<TakeOffer>, _id: String) -> Result<()> {
    let cpi_accounts = TransferChecked {
        mint: _context.accounts.token_mint_a.to_account_info(),
        from: _context.accounts.sender_token_account_a.to_account_info(),
        to: _context
            .accounts
            .recipient_token_account_a
            .to_account_info(),
        authority: _context.accounts.sender_token_account_a.to_account_info(),
    };

    let token_mint_a_key = _context.accounts.token_mint_a.key();
    let maker_key = _context.accounts.offer.maker.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        SWAP_TOKEN_TAG,
        token_mint_a_key.as_ref(),
        maker_key.as_ref(),
        _id.as_bytes(),
        &[_context.bumps.sender_token_account_a],
    ]];
    let cpi_context = CpiContext::new(
        _context.accounts.token_program.to_account_info(),
        cpi_accounts,
    )
    .with_signer(signer_seeds);

    let token_offered_amount = _context.accounts.offer.token_offered_amount;
    let transfer_tx = transfer_checked(
        cpi_context,
        token_offered_amount,
        _context.accounts.token_mint_a.decimals,
    );

    if transfer_tx.is_err() {
        return Err(CustomError::TransferError.into());
    }
    Ok(())
}
