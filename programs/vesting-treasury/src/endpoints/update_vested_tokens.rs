//! Calculates and updates the amount of tokens vested in the vesting schedule.

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

    if accs.vesting.period_type != PeriodType::Monthly {
        return Err(error!(err::arg(
            "The current contract version only supports\
             vesting schedules with monthly periods."
        )));
    }

    accs.vesting.update_vested_tokens(clock_ts)?;
    Ok(())
}
