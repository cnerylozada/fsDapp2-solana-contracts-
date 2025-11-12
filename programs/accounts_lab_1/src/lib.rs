use anchor_lang::prelude::*;

mod constants;
mod instructions;
mod models;
use instructions::*;

declare_id!("6gMKvPBpejRjZmtK2657HsYicjuWhgjJfY8VKmNqboWo");

#[program]
pub mod accounts_lab_1 {
    use super::*;

    pub fn create_vault(_context: Context<CreateVault>, _id: String) -> Result<()> {
        instructions::create_vault::create_vault(_context, _id)
    }

    pub fn delete_vault(_context: Context<DeleteVault>, _id: String) -> Result<()> {
        instructions::delete_vault::delete_vault(_context, _id)
    }
}
