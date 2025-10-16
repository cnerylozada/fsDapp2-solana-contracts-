use crate::errors::Errors::{NotEnoughFunds, TransferError};
use crate::models::SavingAccount;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};

#[derive(Accounts)]
#[instruction(_title: String)]
pub struct SystemTransfer<'info> {
    #[account(mut, seeds = [signer.key().as_ref(), _title.as_bytes()],
    bump = saving_account.bump_seed)]
    pub saving_account: Account<'info, SavingAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn system_transfer(
    _context: Context<SystemTransfer>,
    _title: String,
    _amount: u64,
) -> Result<()> {
    let signer = &_context.accounts.signer;
    let saving_account = &mut _context.accounts.saving_account;

    if signer.lamports() < _amount {
        return Err(NotEnoughFunds.into());
    }
    saving_account.balance += _amount;

    let transfer_ix = transfer(&signer.key(), &saving_account.key(), _amount);
    let transfer_tx = invoke(
        &transfer_ix,
        &[signer.to_account_info(), saving_account.to_account_info()],
    );
    if transfer_tx.is_err() {
        return Err(TransferError.into());
    }
    Ok(())
}
