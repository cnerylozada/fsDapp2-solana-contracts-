use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
use instructions::*;

declare_id!("3JNmFZBLAZDKEAAoYszwRWzV7Ha31JyGsZMsQGLP4jfz");

#[program]
pub mod swap {
    use super::*;

    pub fn create_vault(_context: Context<CreateVault>, _id: String) -> Result<()> {
        instructions::create_vault::create_vault(_context, _id)
    }

    pub fn make_offer(
        _context: Context<MakeOffer>,
        _id: String,
        token_offered_amount: u64,
        token_wanted_amount: u64,
    ) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&_context, token_offered_amount);
        instructions::make_offer::save_offer(
            _context,
            _id,
            token_offered_amount,
            token_wanted_amount,
        )
    }
}
