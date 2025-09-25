use anchor_lang::prelude::*;

declare_id!("DnDa7gtFZNmZMPupeJZVU5TK1FEwJTxBSR9C9cPzxr2G");

#[program]
pub mod simple_transfer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
