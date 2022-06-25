use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, MintTo, Transfer};

declare_id!("DRuSJ4GGtkbxN72vmpoemajxSmgQZgsrfSm6b38vqv2t");

#[program]
pub mod token_contract {
    use super::*;

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        // create the MintTo struct for our context
        let cpi_accounts = MintTo {
            // the account that holds the token info
            mint: ctx.accounts.mint.to_account_info(),
            // the ATA that we want to mint the token into
            to: ctx.accounts.token_account.to_account_info(),
            // the wallet key that owns the mint 
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContext we need for the request
        // cpi_ctx takes in two parameters, the program and the MintTo struct
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // execute anchor's helper functionto mint tokens
        // imported module from anchor_spl
        token::mint_to(cpi_ctx, 10)?;
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
        // creates out transfer struct
        let transfer_instruction = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.from_authority.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // create the context for out tranfer request
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);
        // execute anchor's helper function to transfer tokens
        anchor_spl::token::transfer(cpi_ctx, 5)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
     #[account(mut)]
    // the token that we want to copy to an account
    pub mint: UncheckedAccount<'info>,
    // cpi contex so we can mint our token that we specify
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
     // who we want to mint our tokens to (ATA)
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    // authority to mint tokens to the token account
    pub authority: AccountInfo<'info>,
}


#[derive(Accounts)]
    pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
        // taking tokens from this ATA
    pub from: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
        // sending tokens to this ATA
    to: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from_authority: Signer<'info>,
}

