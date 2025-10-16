use crate::errors::Errors::{NotAllowedOperation, NotEnoughFunds};
use crate::models::SavingAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_title:String)]
pub struct SystemWithdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds = [signer.key().as_ref(), _title.as_bytes()],
    bump = saving_account.bump_seed)]
    pub saving_account: Account<'info, SavingAccount>,
}

pub fn system_withdraw(
    _context: Context<SystemWithdraw>,
    _title: String,
    _amount: u64,
) -> Result<()> {
    let saving_account = &mut _context.accounts.saving_account;
    let signer = &mut _context.accounts.signer;

    if signer.key() != saving_account.owner {
        return Err(NotAllowedOperation.into());
    }
    if _amount > saving_account.balance {
        return Err(NotEnoughFunds.into());
    }

    let source_balance = saving_account.to_account_info();
    **source_balance.try_borrow_mut_lamports()? -= _amount;

    let destiny_balance = signer.to_account_info();
    **destiny_balance.try_borrow_mut_lamports()? += _amount;

    saving_account.balance -= _amount;

    Ok(())
}
