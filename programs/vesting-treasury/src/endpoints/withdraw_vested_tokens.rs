//! Endpoint for users to withdraw vested tokens from [`vesting_vault`] to
//! the [`vestee_wallet`]. Whilst the endpoint is permissionless, only tokens
//! that have been vested and subsequently funded by the administrator or
//! any other agent, will be avaialble for transfer. The endpoint is made
//! permissionless to more easily allow for automation.

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
        mut,
        constraint = vestee_wallet.key() == vesting.vestee_wallet.key()
        @ err::acc("Vestee wallet input does not match the \
         vestee wallet in the vesting account")
    )]
    pub vestee_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handle(ctx: Context<WithdrawVestedTokens>, withdraw_amount: TokenAmount) -> Result<()> {
    let accs = ctx.accounts;
    let signer_bump_seed = *ctx.bumps.get("vesting_signer").unwrap();

    let liability = accs.vesting.get_current_liability();

    if withdraw_amount.amount > liability {
        return Err(error!(err::arg(
            "The amount of tokens to withdraw is bigger than\
            the amount of vested tokens to be withdrawn"
        )));
    }

    if withdraw_amount.amount > accs.vesting.vault_balance.amount {
        return Err(error!(err::arg(
            "The amount of tokens to withdraw is higher \
            than the amount of tokens currently in vault, \
            it seems the vault is partially unfunded"
        )));
    }

    // make token transfers from vesting token vault to vestee wallet
    let signer_seeds = &[
        Vesting::SIGNER_PDA_PREFIX,
        &accs.vesting.key().to_bytes()[..],
        &[signer_bump_seed],
    ];

    token::transfer(
        accs.as_transfer_funds_from_vesting_vault_to_vestee_wallet_context()
            .with_signer(&[&signer_seeds[..]]),
        withdraw_amount.amount,
    )?;

    // Update the vault balance and cumulative withdrawn state
    // Note that we don't need to update unfunded liabilities because the delta
    // by which the vault balance decreases is offset by the delta by which the
    // cumulative withdrawn amount increases (same delta)
    accs.vesting.vault_balance =
        TokenAmount::new(accs.vesting.vault_balance.amount - withdraw_amount.amount);

    accs.vesting.cumulative_withdrawn =
        TokenAmount::new(accs.vesting.cumulative_withdrawn.amount + withdraw_amount.amount);

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
