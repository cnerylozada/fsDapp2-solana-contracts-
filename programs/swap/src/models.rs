use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    #[max_len(13)]
    pub id: String,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_wanted_amount: u64,
    pub token_offered_amount: u64,
    pub bump: u8,
}
