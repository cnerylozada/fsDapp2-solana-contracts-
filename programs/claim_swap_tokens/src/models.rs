use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MainVaultAccount {
    #[max_len(20)]
    pub title: String,
    pub token_mint: Pubkey,
    pub created_at: u64,
    pub bump: u8,
}
