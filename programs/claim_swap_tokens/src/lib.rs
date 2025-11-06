use anchor_lang::prelude::*;
use anchor_spl::{
    self,
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
mod errors;
use errors::Errors::TransferError;
declare_id!("5a2dQe4CbrnJsuxzN7JCGup987km9Vjcnep9GvGwMHjs");

#[program]
pub mod claim_swap_tokens {
    use super::*;

    pub fn transfer_tokens(_context: Context<TransferTokens>, _amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            mint: _context.accounts.token_mint_x.to_account_info(),
            from: _context.accounts.token_account_x.to_account_info(),
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

    pub fn fund_main_vault(_context: Context<FundMainVault>, _amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            mint: _context.accounts.token_mint_x.to_account_info(),
            from: _context.accounts.token_account_x.to_account_info(),
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
}

#[derive(Accounts)]
struct TransferTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_mint_x: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub token_account_x: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub token_x_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
struct FundMainVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_x: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_x,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub token_account_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = token_mint_x,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub token_x_vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
