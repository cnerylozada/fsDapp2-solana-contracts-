use anchor_lang::prelude::*;

use crate::{
    constants::{ACCOUNT_DISCRIMINATOR, PAYER_TAG},
    models::Payment,
};

#[derive(Accounts)]
#[instruction(_id: String)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        space = ACCOUNT_DISCRIMINATOR + Payment::INIT_SPACE,
        payer = signer,
        seeds = [PAYER_TAG, signer.key().as_ref(), _id.as_bytes()],
        bump
    )]
    pub payment: Account<'info, Payment>,

    pub system_program: Program<'info, System>,
}

pub fn create_vault(_context: Context<CreateVault>, _id: String) -> Result<()> {
    let payment = &mut _context.accounts.payment;

    payment.id = _id;
    payment.funder = _context.accounts.signer.key();
    payment.bump = _context.bumps.payment;

    let now = (Clock::get().unwrap().unix_timestamp) as u64;
    payment.create_at = now;

    Ok(())
}
