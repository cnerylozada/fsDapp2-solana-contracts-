use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{ACCOUNT_DISCRIMINATOR, OFFER_TAG, SWAP_TOKEN_TAG},
    instructions::shared::transfer_tokens,
    models::Offer,
};

#[derive(Accounts)]
#[instruction(_id: String)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub sender_token_account_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = token_mint_a,
        token::authority = recipient_token_account_a,
        seeds = [SWAP_TOKEN_TAG, token_mint_a.key().as_ref(), signer.key().as_ref(), _id.as_bytes()],
        bump

    )]
    pub recipient_token_account_a: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = ACCOUNT_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [OFFER_TAG, signer.key().as_ref(), _id.as_bytes()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    pub system_program: Program<'info, System>,
}

pub fn send_offered_tokens_to_vault(
    _context: &Context<MakeOffer>,
    _token_offered_amount: u64,
) -> Result<()> {
    // let cpi_accounts = TransferChecked {
    //     mint: _context.accounts.token_mint_a.to_account_info(),
    //     from: _context.accounts.sender_token_account_a.to_account_info(),
    //     to: _context
    //         .accounts
    //         .recipient_token_account_a
    //         .to_account_info(),
    //     authority: _context.accounts.signer.to_account_info(),
    // };

    // let cpi_context = CpiContext::new(
    //     _context.accounts.token_program.to_account_info(),
    //     cpi_accounts,
    // );

    // let transfer_tx = transfer_checked(
    //     cpi_context,
    //     _token_offered_amount,
    //     _context.accounts.token_mint_a.decimals,
    // );

    // if transfer_tx.is_err() {
    //     return Err(CustomError::TransferError.into());
    // }

    // Ok(())

    transfer_tokens(
        &_context.accounts.token_mint_a,
        &_context.accounts.sender_token_account_a,
        &_context.accounts.recipient_token_account_a,
        &_context.accounts.signer,
        &_context.accounts.token_program,
        _token_offered_amount,
    )
}

pub fn save_offer(
    _context: Context<MakeOffer>,
    _id: String,
    _token_offered_amount: u64,
    _token_wanted_amount: u64,
) -> Result<()> {
    let offer = &mut _context.accounts.offer;

    offer.id = _id;
    offer.maker = _context.accounts.signer.key();
    offer.token_mint_a = _context.accounts.token_mint_a.key();
    offer.token_mint_b = _context.accounts.token_mint_b.key();

    offer.token_offered_amount = _token_offered_amount;
    offer.token_wanted_amount = _token_wanted_amount;

    offer.bump = _context.bumps.offer;

    Ok(())
}
