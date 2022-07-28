//! Funds [`vesting_vault`] account.

use crate::prelude::*;

use anchor_spl::token::{self, Token, TokenAccount};

#[derive(Accounts)]
pub struct FundVestingVault<'info> {
    #[account(mut)]
    pub wallet_authority: Signer<'info>,
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
    #[account(
        mut,
        constraint = vesting_vault.key() == vesting.vault.key()
        @ err::acc("Vault input does not match the vault in the vesting account")
    )]
    pub vesting_vault: Account<'info, TokenAccount>,
    // pub mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = funding_wallet.mint == vesting.mint.key()
        @ err::acc("Funding wallet must be of correct mint")
    )]
    pub funding_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handle(ctx: Context<FundVestingVault>, funding_amount: TokenAmount) -> Result<()> {
    let accs = ctx.accounts;

    // TODO: Check mints match

    token::transfer(
        accs.as_transfer_funds_from_funding_wallet_to_vault_context(),
        funding_amount.amount,
    )?;

    accs.vesting.vesting_vault_balance =
        TokenAmount::new(accs.vesting.vesting_vault_balance.amount + funding_amount.amount);

    // Since more tokens are being added to the vault we need to update how
    // much of the vested tokens is currently unfunded, if any
    accs.vesting.update_unfunded_liability()?;

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
