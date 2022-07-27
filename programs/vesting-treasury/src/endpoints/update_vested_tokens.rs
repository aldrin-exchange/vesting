//! Calculates and updated the amount of tokens vested in the vesting schedule.

use crate::prelude::*;

#[derive(Accounts)]
pub struct UpdateVestedTokens<'info> {
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handle(ctx: Context<UpdateVestedTokens>) -> Result<()> {
    let accs = ctx.accounts;

    let clock_ts = accs.clock.unix_timestamp;

    accs.vesting.update_vested_tokens(clock_ts)?;

    Ok(())
}
