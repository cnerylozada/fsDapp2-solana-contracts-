use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Transfer failed")]
    TransferError,
    #[msg("User has already claimed the maximum number of tokens.")]
    ClaimLimitExceeded,
}
