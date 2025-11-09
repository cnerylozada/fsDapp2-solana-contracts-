use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ClaimRecord {
    pub user: Pubkey,
    pub total_claimed: u64,
    pub bump: u8,
}
