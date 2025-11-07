use anchor_lang::prelude::*;
use anchor_spl::{
    self,
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
mod errors;
use errors::CustomError::TransferError;
mod instructions;
use instructions::*;
declare_id!("5a2dQe4CbrnJsuxzN7JCGup987km9Vjcnep9GvGwMHjs");

#[program]
pub mod claim_swap_tokens {
    use super::*;

    pub fn transfer_tokens(_context: Context<TransferTokens>, _amount: u64) -> Result<()> {
        instructions::transfer_tokens(_context, _amount)
    }

    pub fn create_main_vault(_context: Context<CreateMainVault>) -> Result<()> {
        Ok(())
    }
    pub fn fund_main_vault(_context: Context<FundMainVault>, _amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            mint: _context.accounts.token_mint_x.to_account_info(),
            from: _context.accounts.sender_token_x_account.to_account_info(),
            to: _context.accounts.token_x_vault.to_account_info(),
            authority: _context.accounts.signer.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            _context.accounts.token_program.to_account_info(),
            transfer_accounts,
        );

        let transfer_tx = transfer_checked(
            cpi_context,
            _amount,
            _context.accounts.token_mint_x.decimals,
        );

        if transfer_tx.is_err() {
            return Err(TransferError.into());
        }

        Ok(())
    }

    pub fn claim_tokens(_context:Context<ClaimTokens>, _amount: u64) -> Result<()>{
        let signer_seeds: &[&[&[u8]]] = &[&[b"token", &[_context.bumps.tokenx_x_vault]]];

        let transfer_accounts = TransferChecked {
            mint: _context.accounts.token_mint_x.to_account_info(),
            from: _context.accounts.tokenx_x_vault.to_account_info(),
            to: _context.accounts.recipient_token_x_account.to_account_info(),
            authority: _context.accounts.tokenx_x_vault.to_account_info(),
        };
        let cpi_context = CpiContext::new(
            _context.accounts.token_program.to_account_info(),
            transfer_accounts
        ).with_signer(signer_seeds);
        
        let transfer_tx = transfer_checked(
            cpi_context,
            _amount,
            _context.accounts.token_mint_x.decimals
        );

        if transfer_tx.is_err() {
            return Err(TransferError.into());
        }

        Ok(())
    }
}



#[derive(Accounts)]
struct CreateMainVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint_x: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        token::mint = token_mint_x,
        token::authority = token_x_vault,
        seeds = [b"token"],
        bump    
    )]
    pub token_x_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
struct FundMainVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint_x: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub sender_token_x_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = token_mint_x,
        token::authority = token_x_vault,
        seeds = [b"token"],
        bump
    )]
    pub token_x_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
struct ClaimTokens<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint_x: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = token_mint_x,
        token::authority = tokenx_x_vault,
        seeds = [b"token"],
        bump
    )]
    pub tokenx_x_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint_x,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub recipient_token_x_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

}