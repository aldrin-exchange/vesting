//! If the [`Vesting`] is fully vested and has no tokens that remain to be
//! withdrawn, then the account is empty and can be closed without losing
//! funds.
use crate::prelude::*;

#[derive(Accounts)]
pub struct CloseVestingSchedule<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        constraint = vesting.admin == admin.key()
            @ err::acc("Admin does not own this vesting account"),
        close = admin,
    )]
    pub vesting: Account<'info, Vesting>,
}

pub fn handle(ctx: Context<CloseVestingSchedule>) -> Result<()> {
    let vesting = &ctx.accounts.vesting;

    if vesting.cumulative_vested_amount < vesting.total_vesting_amount {
        return Err(error!(err::acc("This vesting account is not fully vested")));
    }

    if vesting.cumulative_vested_amount > vesting.cumulative_withdrawn_amount {
        return Err(error!(err::acc(
            "This vested tokens of this vesting account are not fully withdrawn"
        )));
    }

    Ok(())
}
