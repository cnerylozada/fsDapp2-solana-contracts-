use anchor_lang::prelude::*;
declare_id!("DnDa7gtFZNmZMPupeJZVU5TK1FEwJTxBSR9C9cPzxr2G");

mod errors;
mod instructions;
mod models;
use instructions::*;

#[program]
pub mod simple_transfer {
    use super::*;

    pub fn create_account(_context: Context<CreateAccount>, _title: String) -> Result<()> {
        create_account::create_account(_context, _title)
    }

    pub fn system_transfer(
        _context: Context<SystemTransfer>,
        _title: String,
        _amount: u64,
    ) -> Result<()> {
        system_transfer::system_transfer(_context, _title, _amount)
    }
}
