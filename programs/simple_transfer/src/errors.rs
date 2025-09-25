use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Not enough funds")]
    NotEnoughFunds,
    #[msg("Transfer failed")]
    TransferError,
}
