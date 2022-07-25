//! Changes vestee wallet in [`Vesting`] account. The admin should be able
//! to change the vestee wallet such that in case the vestee wallet becomes
//! compromised, the admin is able to target a different vestee wallet for a
//! given vesting account.
use crate::prelude::*;

use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct ChangeVesteeWallet<'info> {
    pub admin: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
    pub vestee_wallet_new: Account<'info, TokenAccount>,
}

pub fn handle(ctx: Context<ChangeVesteeWallet>) -> Result<()> {
    let accs = ctx.accounts;

    if accs.vesting.admin != accs.admin.key() {
        return Err(error!(TreasuryError::VestingAdminMismatch));
    }

    if accs.vesting.mint != accs.vestee_wallet_new.mint {
        return Err(error!(err::acc(
            "The new vestee wallet mint must be of correct mint"
        )));
    }

    if accs.vesting.vestee_wallet == accs.vestee_wallet_new.key() {
        return Err(error!(err::acc(
            "The new vestee wallet is the same as the current vestee wallet"
        )));
    }
    accs.vesting.vestee_wallet = accs.vestee_wallet_new.key();

    Ok(())
}
