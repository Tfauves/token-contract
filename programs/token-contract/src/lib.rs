use anchor_lang::prelude::*;

declare_id!("DRuSJ4GGtkbxN72vmpoemajxSmgQZgsrfSm6b38vqv2t");

#[program]
pub mod token_contract {
    use super::*;

    pub fn mint_tokens(ctx: Context<MintToken>) -> Result<()> {
        // create the MintTo struct for our context
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContext we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // execute anchor's helper functionto mint tokens
        token::mint_to(cpi_ctx, 10)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub MintToken<'info> {
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
    pub payer: Signer<'info>,

}
