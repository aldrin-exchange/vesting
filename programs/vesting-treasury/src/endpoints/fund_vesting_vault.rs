//! Funds [`vesting_vault`] account.

use crate::prelude::*;

use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct FundVestingVault<'info> {
    #[account(mut)]
    pub wallet_authority: Signer<'info>,
    #[account(zero)]
    pub vesting: Account<'info, Vesting>,
    #[account(
        constraint = vesting_vault.mint == vesting.mint.key()
        @ err::acc("Vestee wallet must be of correct mint")
    )]
    pub vesting_vault: Account<'info, TokenAccount>,
    // pub mint: Account<'info, Mint>,
    #[account(
        constraint = funding_wallet.mint == vesting.mint.key()
        @ err::acc("Vestee wallet must be of correct mint")
    )]
    pub funding_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handle(ctx: Context<FundVestingVault>, funding_amount: TokenAmount) -> Result<()> {
    let accs = ctx.accounts;

    // Check mints match

    token::transfer(
        accs.as_transfer_funds_from_funding_wallet_to_vault_context(),
        funding_amount.amount,
    )?;

    Ok(())
}
impl<'info> FundVestingVault<'info> {
    fn as_transfer_funds_from_funding_wallet_to_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = token::Transfer {
            from: self.funding_wallet.to_account_info(),
            to: self.vesting_vault.to_account_info(),
            authority: self.wallet_authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
