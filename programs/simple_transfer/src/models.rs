use anchor_lang::prelude::*;

pub const ACCOUNT_DISCRIMINATOR: usize = 8;

#[account]
#[derive(InitSpace)]
pub struct SavingAccount {
    #[max_len(20)]
    pub title: String,
    pub created_at: u64,
    pub bump_seed: u8,
    pub balance: u64,
}
