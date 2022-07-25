//! Changes vestee wallet in [`Vesting`] account. The admin should be able
//! to change the vestee wallet such that in case the vestee wallet becomes
//! compromised, the admin is able to target a different vestee wallet for a
//! given vesting account.
use crate::prelude::*;

use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct ChangeVestingWallet<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
    pub vestee_wallet_old: Account<'info, TokenAccount>,
    #[account(
        constraint = vestee_wallet_old.mint == vestee_wallet_new.mint
        @ err::acc("New cestee wallet must be of correct mint")
    )]
    pub vestee_wallet_new: Account<'info, TokenAccount>,
}

pub fn handle(ctx: Context<ChangeVestingWallet>) -> Result<()> {
    let accs = ctx.accounts;

    if accs.vesting.admin != accs.admin.key() {
        return Err(error!(TreasuryError::VestingAdminMismatch));
    }

    accs.vesting.vestee_wallet = accs.vestee_wallet_new.key();

    Ok(())
}
