use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Payment {
    #[max_len(13)]
    pub id: String,
    pub funder: Pubkey,
    pub bump: u8,
    pub create_at: u64,
    pub is_disabled: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Aux {
    #[max_len(13)]
    pub last_id: String,
    pub deleted_at: u64,
    pub bump: u8,
}
