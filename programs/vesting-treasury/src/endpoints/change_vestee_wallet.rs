//! Changes vestee wallet in [`Vesting`] account. The admin should be able
//! to change the vestee wallet such that in case the vestee wallet becomes
//! compromised, the admin is able to target a different vestee wallet for a
//! given vesting account.

use crate::prelude::*;

use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct ChangeVesteeWallet<'info> {
    #[account(
        constraint = admin.key() == vesting.admin
        @ err::acc("Vesting admin does not match the provided signer")
    )]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
    #[account(
        constraint = vestee_wallet_new.mint == vesting.mint
        @ err::acc("The new vestee wallet mint must be of correct mint")
    )]
    pub vestee_wallet_new: Account<'info, TokenAccount>,
}

pub fn handle(ctx: Context<ChangeVesteeWallet>) -> Result<()> {
    let accs = ctx.accounts;

    if accs.vesting.vestee_wallet == accs.vestee_wallet_new.key() {
        return Err(error!(err::acc(
            "The new vestee wallet is the same as the current vestee wallet"
        )));
    }
    accs.vesting.vestee_wallet = accs.vestee_wallet_new.key();

    Ok(())
}
