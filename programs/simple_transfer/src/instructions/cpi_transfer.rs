use crate::errors::Errors::TransferError;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct CPITransfer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds = [b"vault", signer.key().as_ref()], bump)]
    pub vault_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn cpi_transfer(_context: Context<CPITransfer>, _amount: u64) -> Result<()> {
    let transfer_accounts = Transfer {
        from: _context.accounts.signer.to_account_info(),
        to: _context.accounts.vault_account.to_account_info(),
    };
    let cpi_context = CpiContext::new(
        _context.accounts.system_program.to_account_info(),
        transfer_accounts,
    );
    let transfer_tx = transfer(cpi_context, _amount);
    if transfer_tx.is_err() {
        return Err(TransferError.into());
    }
    Ok(())
}
