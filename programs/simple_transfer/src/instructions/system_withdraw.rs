use crate::errors::Errors::NotEnoughFunds;
use crate::models::SavingAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SystemWithdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub saving_account: Account<'info, SavingAccount>,
}

pub fn system_withdraw(_context: Context<SystemWithdraw>, _amount: u64) -> Result<()> {
    let saving_account = &mut _context.accounts.saving_account;
    if _amount > saving_account.balance {
        return Err(NotEnoughFunds.into());
    }

    saving_account.balance -= _amount;
    Ok(())
}
