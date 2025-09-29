use anchor_lang::prelude::*;
mod errors;

declare_id!("DnDa7gtFZNmZMPupeJZVU5TK1FEwJTxBSR9C9cPzxr2G");

const ACCOUNT_DISCRIMINATOR: usize = 8;

#[program]
pub mod simple_transfer {
    use super::*;

    pub fn create_account(_context: Context<CreateAccount>, _title: String) -> Result<()> {
        let saving_account = &mut _context.accounts.saving_account;
        saving_account.title = _title;
        let now = (Clock::get().unwrap().unix_timestamp) as u64;
        saving_account.created_at = now;
        saving_account.bump_seed = _context.bumps.saving_account;

        Ok(())
    }

    pub fn deposit(_context: Context<Deposit>, _title: String, _amount: u64) -> Result<()> {
        let signer = &_context.accounts.signer;
        let saving_account = &_context.accounts.saving_account;

        if signer.lamports() < _amount {
            return Err(errors::Errors::NotEnoughFunds.into());
        }

        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            &signer.key(),
            &saving_account.key(),
            _amount,
        );
        let transfer_tx = anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[signer.to_account_info(), saving_account.to_account_info()],
        );
        if transfer_tx.is_err() {
            return Err(errors::Errors::TransferError.into());
        }

        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(_title: String)]
struct CreateAccount<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(init, payer = signer,
        space = ACCOUNT_DISCRIMINATOR + SavingAccount::INIT_SPACE,
        seeds = [signer.key().as_ref(), _title.as_bytes()], bump)]
    saving_account: Account<'info, SavingAccount>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_title: String)]
struct Deposit<'info> {
    #[account(mut, seeds = [signer.key().as_ref(), _title.as_bytes()],
    bump = saving_account.bump_seed)]
    saving_account: Account<'info, SavingAccount>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
struct SavingAccount {
    #[max_len(20)]
    title: String,
    created_at: u64,
    bump_seed: u8,
}
