//! Calculates and updates the amount of tokens vested in the vesting schedule.
//! The endpoint uses the solana clock account to access the runtime clock and
//! compare it against the vesting dates to calculate how many periods have
//! vested. Based on the amount of vested periods the endpoint then computes
//! the pro-rata amount of tokens that are vested.

use crate::prelude::*;
use std::collections::HashSet;

#[derive(Accounts)]
pub struct UpdateVestedTokens<'info> {
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
}

pub fn handle(ctx: Context<UpdateVestedTokens>) -> Result<()> {
    let accs = ctx.accounts;

    let clock_ts = TimeStamp::current()?;
    // let clock_ts = Clock::get()?.unix_timestamp;

    let supported_types: HashSet<PeriodType> =
        HashSet::from([PeriodType::Monthly, PeriodType::Daily]);

    if !supported_types.contains(&accs.vesting.period_type) {
        return Err(error!(err::arg(
            "The current contract version only supports \
            vesting schedules with daily or monthly periods"
        )));
    }

    accs.vesting.update_vested_tokens(clock_ts.time)?;

    // Since more tokens may be vested we need to update how much of
    // those vested tokens is currently unfunded
    accs.vesting.update_unfunded_liability()?;

    Ok(())
}
