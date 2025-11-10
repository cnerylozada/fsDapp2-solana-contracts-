use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::errors::CustomError;

pub fn transfer_tokens<'info>(
    _mint: &InterfaceAccount<'info, Mint>,
    _from: &InterfaceAccount<'info, TokenAccount>,
    _to: &InterfaceAccount<'info, TokenAccount>,
    _authority: &Signer<'info>,
    _token_program: &Interface<'info, TokenInterface>,
    _amount: u64,
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        mint: _mint.to_account_info(),
        from: _from.to_account_info(),
        to: _to.to_account_info(),
        authority: _authority.to_account_info(),
    };

    let cpi_context = CpiContext::new(_token_program.to_account_info(), cpi_accounts);

    let transfer_tx = transfer_checked(cpi_context, _amount, _mint.decimals);

    if transfer_tx.is_err() {
        return Err(CustomError::TransferError.into());
    }

    Ok(())
}
