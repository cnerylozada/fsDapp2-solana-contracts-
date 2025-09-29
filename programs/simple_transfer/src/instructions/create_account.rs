use crate::models::{SavingAccount, ACCOUNT_DISCRIMINATOR};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_title: String)]
pub struct CreateAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer,
        space = ACCOUNT_DISCRIMINATOR + SavingAccount::INIT_SPACE,
        seeds = [signer.key().as_ref(), _title.as_bytes()], bump)]
    pub saving_account: Account<'info, SavingAccount>,

    pub system_program: Program<'info, System>,
}

pub fn create_account(_context: Context<CreateAccount>, _title: String) -> Result<()> {
    let saving_account = &mut _context.accounts.saving_account;
    saving_account.title = _title;
    let now = (Clock::get().unwrap().unix_timestamp) as u64;
    saving_account.created_at = now;
    saving_account.bump_seed = _context.bumps.saving_account;

    Ok(())
}
