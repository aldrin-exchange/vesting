use crate::prelude::*;

/// Computes the amount of periods in between two dates in the same year.
/// As an example:
/// cliff_dt = 15/03/2020
/// current_dt = 20/09/2020
///
/// First we confirm that current_dt is not before the cliff_dt, otherwise
/// we return zero periods. Otherwise, we subtract the amount of months between
/// each date and add 1 period in the curren_dt day is equal or after the
/// cliff_dt day. In our example the curent day is 20 which is superior or
/// equal to 15. We therefore count that month as a period, hence why we add 1.
pub fn compute_periods_from_cliff_to_current_dt(
    cliff_dt: DateTime<Utc>,
    current_dt: DateTime<Utc>,
) -> u32 {
    if current_dt.month() < cliff_dt.month() {
        return 0;
    }

    current_dt.month()
        - cliff_dt.month()
        - if current_dt.day() < cliff_dt.day() {
            1
        } else {
            0
        }
}

/// Computes the amount of periods the cliff_dt datetime till the end of the
/// year (eoy). Since the cliff_dt datetime day is guaranteed to match the day in
/// which the vesting schedule started, we know that we can easily subtract the
/// total number of periods in a year (if monthly this means 12) by the period
/// refering the to the cliff_dt (i.e. month)
pub fn compute_periods_from_cliff_to_eoy(cliff_dt: DateTime<Utc>) -> u32 {
    12 - cliff_dt.month()
}

/// Computes the amount of periods from the beginning of the current year (boy)
/// to the current_dt datetime.
///
/// When PeriodType is Monthly the number or periods will be the current_dt
/// month minus 1 in case the current day is inferior to the cliff_dt day.
/// Since the cliff_dt datetime day is guaranteed to match the day in
/// which the vesting schedule started, we we look at the cliff_dt day and
/// compare it to the current day to infer if we should count or not with the
/// current period, hence the substrating by 1 means that we are taking our the
/// current period because this one has not finished.
pub fn compute_periods_from_boy_to_current_dt(
    cliff_dt: DateTime<Utc>,
    current_dt: DateTime<Utc>,
) -> u32 {
    current_dt.month()
        - if current_dt.day() >= cliff_dt.day() {
            0
        } else {
            1
        }
}

/// Computes the amount of periods in a set of full years, represented by
/// `delta_years` - 1. In the case the PeriodType is Monthly, we simply
/// multiply the number of years by 12
pub fn compute_periods_in_full_years(delta_years: u32) -> u32 {
    (delta_years - 1) * 12
}
