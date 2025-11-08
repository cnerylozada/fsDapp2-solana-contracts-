use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
use instructions::*;
declare_id!("5a2dQe4CbrnJsuxzN7JCGup987km9Vjcnep9GvGwMHjs");

#[program]
pub mod claim_swap_tokens {
    use super::*;

    pub fn transfer_tokens(_context: Context<TransferTokens>, _amount: u64) -> Result<()> {
        instructions::transfer_tokens(_context, _amount)
    }

    pub fn create_main_vault(_context: Context<CreateMainVault>, _title: String) -> Result<()> {
        instructions::create_main_vault(_context, _title)
    }
    pub fn fund_main_vault(
        _context: Context<FundMainVault>,
        _title: String,
        _amount: u64,
    ) -> Result<()> {
        instructions::fund_main_vault(_context, _title, _amount)
    }

    pub fn claim_tokens(
        _context: Context<ClaimTokens>,
        _title: String,
        _amount: u64,
    ) -> Result<()> {
        instructions::claim_tokens(_context, _title, _amount)
    }
}
