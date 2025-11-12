use anchor_lang::prelude::*;

use crate::{
    constants::{ACCOUNT_DISCRIMINATOR, AUX_TAG, PAYER_TAG},
    models::{Aux, Payment},
};

#[derive(Accounts)]
#[instruction(_id: String)]
pub struct DeleteVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [PAYER_TAG, signer.key().as_ref(), _id.as_bytes()],
        bump = payment.bump,
        close = signer
    )]
    pub payment: Account<'info, Payment>,

    #[account(
        init,
        space = ACCOUNT_DISCRIMINATOR + Aux::INIT_SPACE,
        payer = signer,
        seeds = [AUX_TAG, signer.key().as_ref(), _id.as_bytes()],
        bump
    )]
    pub aux: Account<'info, Aux>,

    pub system_program: Program<'info, System>,
}

pub fn delete_vault(_context: Context<DeleteVault>, _id: String) -> Result<()> {
    let aux = &mut _context.accounts.aux;

    aux.last_id = _context.accounts.payment.id.to_string();
    aux.bump = _context.bumps.aux;

    let now = (Clock::get().unwrap().unix_timestamp) as u64;
    aux.deleted_at = now;

    Ok(())
}
