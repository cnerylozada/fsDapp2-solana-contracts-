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

    pub fn deposit(_context: Context<Deposit>, _title: String, _amount: u64) -> Result<()> {
        deposit::deposit(_context, _title, _amount)
    }
}
