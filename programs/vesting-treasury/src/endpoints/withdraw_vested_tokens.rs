//! TODO

use crate::prelude::*;

use anchor_spl::token::{self, Token, TokenAccount};

#[derive(Accounts)]
pub struct WithdrawVestedTokens<'info> {
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
    #[account(
        mut,
        constraint = vesting_vault.key() == vesting.vault.key()
        @ err::acc("Vault input does not match the vault in the vesting account")
    )]
    pub vesting_vault: Account<'info, TokenAccount>,
    /// CHECK: UNSAFE_CODES.md#signer
    #[account(
        seeds = [Vesting::SIGNER_PDA_PREFIX, vesting.key().as_ref()],
        bump
    )]
    pub vesting_signer: AccountInfo<'info>,
    #[account(
        constraint = vestee_wallet.key() == vesting.vestee_wallet.key()
        @ err::acc("Vestee wallet input does not match the\
         vestee wallet in the vesting account")
    )]
    pub vestee_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handle(ctx: Context<WithdrawVestedTokens>, withdraw_amount: TokenAmount) -> Result<()> {
    let accs = ctx.accounts;

    if withdraw_amount.amount > accs.vesting.vesting_vault_balance.amount {
        return Err(error!(err::arg(
            "The amount of tokens to withdraw is higher \
            thatn the amount of tokens currently in vault"
        )));
    }

    token::transfer(
        accs.as_transfer_funds_from_vesting_vault_to_vestee_wallet_context(),
        withdraw_amount.amount,
    )?;

    accs.vesting.vesting_vault_balance =
        TokenAmount::new(accs.vesting.vesting_vault_balance.amount - withdraw_amount.amount);

    Ok(())
}
impl<'info> WithdrawVestedTokens<'info> {
    fn as_transfer_funds_from_vesting_vault_to_vestee_wallet_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = token::Transfer {
            from: self.vesting_vault.to_account_info(),
            to: self.vestee_wallet.to_account_info(),
            authority: self.vesting_signer.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
