use crate::prelude::*;
use chrono::Duration;
use std::mem;

use crate::periods::monthly;

#[derive(Default, Debug)]
#[account]
pub struct Vesting {
    /// The authority of this Vesting account.
    pub admin: Pubkey,
    /// The vestee wallet of this Vesting account, that should receive the
    /// vested funds.
    pub vestee_wallet: Pubkey,
    /// The mint of the SPL token locked up.
    pub mint: Pubkey,
    /// Address of the account's token vault.
    pub vault: Pubkey,
    /// The total amount that will vest over time
    pub total_vesting: TokenAmount,
    /// Cumulative amount that vested thus far
    pub cumulative_vested: TokenAmount,
    /// Cumulative amount withdrawn thus far
    pub cumulative_withdrawn: TokenAmount,
    /// Current amount sitting in the vesting vault
    pub vault_balance: TokenAmount,
    /// The unfunded liability is the amount of vested tokens that a user
    /// is already allowed to withdraw but are still not available in the
    /// vesting vault, therefore constituting a liability on behalf of
    /// the funder.
    pub unfunded_liability: TokenAmount,
    /// The start time in Unix Timestamp of the vesting period
    pub start_ts: TimeStamp,
    /// The amount of periods in total in the vesting schedule, where a period
    /// represents a different timestamp depending on the period_type
    pub total_periods: u64,
    /// The amount of periods in the cliff part of the schedule, where a period
    /// represents a different timestamp depending on the period_type
    pub cliff_periods: u64,
    /// The type of period (i.e. Monthly, Yearly, etc.) of the vesting
    /// schedule. This is required for computing vesting schedules depending
    /// on different base periods
    pub period_type: PeriodType,
}

impl Vesting {
    pub const VAULT_PREFIX: &'static [u8; 5] = b"vault";
    pub const SIGNER_PDA_PREFIX: &'static [u8; 6] = b"signer";

    pub fn space() -> usize {
        let discriminant = 8;
        let admin = 32;
        let vestee_wallet = 32;
        let mint = 32;
        let vault = 32;

        let total_vesting = mem::size_of::<TokenAmount>();
        let cumulative_vested = mem::size_of::<TokenAmount>();
        let cumulative_withdrawn = mem::size_of::<TokenAmount>();
        let vesting_vault_balance = mem::size_of::<TokenAmount>();
        let unfunded_liabilities = mem::size_of::<TokenAmount>();

        let start_ts = mem::size_of::<i32>();

        let total_periods = mem::size_of::<u64>();
        let cliff_periods = mem::size_of::<u64>();
        let period_type = mem::size_of::<PeriodType>();

        discriminant
            + admin
            + vestee_wallet
            + mint
            + vault
            + mint
            + total_vesting
            + cumulative_vested
            + cumulative_withdrawn
            + vesting_vault_balance
            + unfunded_liabilities
            + start_ts
            + total_periods
            + cliff_periods
            + period_type
    }

    /// Updates the field `cumulative_vested` in [`Vesting`] struct based
    /// on the amount of days that have passed. The method receives the
    /// argument `clock_ts`, which stands for clock timestamp. In the endpoint
    /// `updated_vested_tokens` we call this method with `clock_ts` being the
    /// the current timestamp given by the runtime.
    ///
    /// Vesting schedules have a cliff period following by a period where the
    /// schedule vests periodically, usually monthly or daily. The periodicity
    /// is given by the `self.period_type`. As of this contract version only the
    /// type `Monthly` or `Daily` are supported by the endpoint that calls this
    /// method.
    ///
    /// If we find ourselves before the end of the cliff period, the amount of
    /// tokens vested is nill, therefore we perform an early return and do not
    /// update state. If we find ourselves after the end of the full vesting
    /// period then all the tokens will be vested and the state updated
    /// accordingly.
    pub fn update_vested_tokens(&mut self, clock_ts: i64) -> Result<()> {
        // Converting timestamps to datetimes
        let current_dt: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(clock_ts, 0), Utc);

        let start_dt: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(self.start_ts.time, 0), Utc);

        // cliff_dt marks the end of the cliff period
        // end_dt marks the end of the full vesting period
        let cliff_dt = self.shift_periods(start_dt, self.cliff_periods)?;
        let end_dt = self.shift_periods(start_dt, self.total_periods)?;

        if current_dt < cliff_dt {
            msg!(
                "We are still in the cliff period and \
                therefore there are no vested tokens yet"
            );
            return Ok(());
        }

        if current_dt >= end_dt {
            msg!("All tokens are fully vested");
            self.cumulative_vested = self.total_vesting;
            return Ok(());
        }

        let delta_periods = self.compute_delta_periods(current_dt, cliff_dt)?;

        // (cliff_periods + Δperiods) * total_amount / total_periods
        let cumulative_vested = Decimal::from(self.cliff_periods)
            .try_add(Decimal::from(delta_periods))?
            .try_div(Decimal::from(self.total_periods))?
            .try_mul(Decimal::from(self.total_vesting))?
            .try_floor()?;

        self.cumulative_vested = TokenAmount::new(cumulative_vested);

        Ok(())
    }

    /// This method computes the amount of periods between two dates. The
    /// operations used to compute the result depend on the type of period
    /// defined by the enum PeriodType.
    ///
    /// The current contract supports the PeriodType of `Daily` and `Monthly`.
    ///
    /// If the type is daily then the calculation is simply the difference in
    /// full days between the cliff date and the current date. Note that the
    /// current date is guaranteed to be higher or equal to the cliff date since
    /// we only call this method after confirm that such condition holds.
    ///
    /// If the type is monthly then depending if both dates are in the same year
    /// or if they are years apart from each other the method will break down
    /// the calcualtion in three steps. The first year, the years in between
    /// and the last year, and will call functions for each step
    pub fn compute_delta_periods(
        &mut self,
        current_dt: DateTime<Utc>,
        cliff_dt: DateTime<Utc>,
    ) -> Result<u64> {
        if current_dt < cliff_dt {
            return Err(error!(err::arg(
                "This function should never be called when current_dt < cliff_dt"
            )));
        }

        match self.period_type {
            PeriodType::Daily => {
                let delta_periods = current_dt
                    .date()
                    .signed_duration_since(cliff_dt.date())
                    .num_days();

                Ok(delta_periods as u64)
            }
            PeriodType::Monthly => {
                let delta_years = (current_dt.year() - cliff_dt.year()) as u32;

                // We want to compute the amount of periods between two dates.
                // Depending on the difference in years between the two dates we will
                // have to perform different steps, described below
                let delta_periods = match delta_years {
                    // This means that both dates are in the same year
                    // e.g. 15/03/2020 & 20/09/2020
                    0 => monthly::compute_periods_from_cliff_to_current_dt(cliff_dt, current_dt),
                    // This means that both dates are one year apart
                    // e.g. 15/03/2020 & 20/09/2021
                    // We therefore perform two distinct operations, for the first year
                    // and the second one
                    1 => {
                        // Periods from 15/03/2020 to 31/12/2020
                        monthly::compute_periods_from_cliff_to_eoy(cliff_dt)
                            // Periods from 01/01/2021 to 20/09/2021
                            + monthly::compute_periods_from_boy_to_current_dt(cliff_dt, current_dt)
                    }
                    // This means that both dates are at least two years apart
                    // e.g. 15/03/2020 & 20/09/2024
                    // We therefore perform two distinct operations, for the first year,
                    // for the years in the middle and the last year
                    _ => {
                        // Periods from 15/03/2020 to 31/12/2020
                        monthly::compute_periods_from_cliff_to_eoy(cliff_dt)
                            // Periods from 01/01/2020 to 31/12/2023
                            + monthly::compute_periods_in_full_years(delta_years)
                            // Periods from 01/01/2024 to 20/09/2024
                            + monthly::compute_periods_from_boy_to_current_dt(cliff_dt, current_dt)
                    }
                };
                Ok(delta_periods as u64)
            }
            _ => Err(error!(err::acc(
                "Current program only supports Daily or Monthly PeriodType"
            ))),
        }
    }

    /// Shifts a date according to the period defined. If the period defined in
    /// the vesting account is `Monthly` then it will shift the date by n months
    /// and if the period is `Daily` it will shift by n days, where n is the
    /// argument `periods`
    pub fn shift_periods(&mut self, date: DateTime<Utc>, periods: u64) -> Result<DateTime<Utc>> {
        match self.period_type {
            PeriodType::Daily => date
                .checked_add_signed(Duration::days(periods as i64))
                .ok_or_else(|| error!(TreasuryError::InvariantViolation)),
            PeriodType::Monthly => Ok(shift_months(date, periods as i32)),
            _ => Err(error!(err::acc(
                "Current program only supports Daily or Monthly PeriodType"
            ))),
        }
    }

    /// It updates unfunded liability of the vesting account. The unfunded
    /// is the amount of vested tokens that a user is already allowed to
    /// withdraw but are still not available in the vesting vault, therefore
    /// constituting a liability on behalf of the funder.
    ///
    /// To calculate the unfunded liabilities we first compute the liability,
    /// which is simply the difference between what has vested and what has
    /// been withdrawn. From that liability we then compare it with the vesting
    /// vault balance to determine if there is any unfunded amount.
    pub fn update_unfunded_liability(&mut self) -> Result<()> {
        // Cum withdrawn can never be bigger than cum vested by design
        let liability = Decimal::from(self.cumulative_vested)
            .try_sub(Decimal::from(self.cumulative_withdrawn))?
            .try_round()?;

        if self.vault_balance.amount >= liability {
            // Because the whole current liability is funded or overfunded
            return Ok(());
        }
        let unfunded_liability = liability - self.vault_balance.amount;

        self.unfunded_liability = TokenAmount::new(unfunded_liability);

        Ok(())
    }

    pub fn get_current_liability(&mut self) -> u64 {
        self.cumulative_vested.amount - self.cumulative_withdrawn.amount
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PeriodType {
    Daily,
    Monthly,
    Quarterly,
    SemiAnnually,
    Yearly,
}

impl Default for PeriodType {
    fn default() -> Self {
        PeriodType::Monthly
    }
}

impl PeriodType {
    pub fn from_u32(value: u32) -> Result<PeriodType> {
        match value {
            1 => Ok(PeriodType::Daily),
            2 => Ok(PeriodType::Monthly),
            3 => Ok(PeriodType::Quarterly),
            4 => Ok(PeriodType::SemiAnnually),
            5 => Ok(PeriodType::Yearly),
            _ => Err(error!(err::arg("The period type enumeration is invalid"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_update_vested_tokens_if_before_cliff_date() -> Result<()> {
        let clock = TimeStamp::new_dt(Utc.ymd(2021, 6, 14));

        let mut vesting = Vesting {
            total_vesting: TokenAmount::new(10_000),
            cumulative_vested: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 48,
            cliff_periods: 12,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;

        // Check that nothing has changed
        assert_eq!(vesting.cumulative_vested, TokenAmount::new(0));
        assert_eq!(vesting.total_vesting, TokenAmount::new(10_000));
        assert_eq!(vesting.cliff_periods, 12);
        assert_eq!(vesting.total_periods, 48);
        assert_eq!(vesting.start_ts, TimeStamp::new_dt(Utc.ymd(2020, 6, 15)));

        Ok(())
    }

    #[test]
    fn it_updates_vested_tokens_if_is_cliff_date() -> Result<()> {
        let clock = TimeStamp::new_dt(Utc.ymd(2021, 6, 15));

        let mut vesting = Vesting {
            total_vesting: TokenAmount::new(10_000),
            cumulative_vested: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 48,
            cliff_periods: 12,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;

        // Check that cumulative vested amount is correct
        assert_eq!(vesting.cumulative_vested, TokenAmount::new(2_500));

        // Check that nothing else has changed
        assert_eq!(vesting.total_vesting, TokenAmount::new(10_000));
        assert_eq!(vesting.cliff_periods, 12);
        assert_eq!(vesting.total_periods, 48);
        assert_eq!(vesting.start_ts, TimeStamp::new_dt(Utc.ymd(2020, 6, 15)));

        Ok(())
    }

    #[test]
    fn it_updates_vested_tokens_when_no_cliff() -> Result<()> {
        let clock = TimeStamp::new_dt(Utc.ymd(2020, 7, 15));

        let mut vesting = Vesting {
            total_vesting: TokenAmount::new(10_000),
            cumulative_vested: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 48,
            cliff_periods: 0,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;

        // Check that cumulative vested amount is correct
        assert_eq!(vesting.cumulative_vested, TokenAmount::new(208));

        Ok(())
    }

    #[test]
    fn it_updates_vested_tokens_if_after_cliff_date() -> Result<()> {
        let clock = TimeStamp::new_dt(Utc.ymd(2022, 6, 15));

        let mut vesting = Vesting {
            total_vesting: TokenAmount::new(10_000),
            cumulative_vested: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 48,
            cliff_periods: 12,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;

        // Check that cumulative vested amount is correct
        assert_eq!(vesting.cumulative_vested, TokenAmount::new(5_000));

        // Check that nothing else has changed
        assert_eq!(vesting.total_vesting, TokenAmount::new(10_000));
        assert_eq!(vesting.cliff_periods, 12);
        assert_eq!(vesting.total_periods, 48);
        assert_eq!(vesting.start_ts, TimeStamp::new_dt(Utc.ymd(2020, 6, 15)));

        Ok(())
    }

    #[test]
    fn it_updates_vested_tokens_when_whole_vesting_is_cliff() -> Result<()> {
        let mut clock = TimeStamp::new_dt(Utc.ymd(2021, 6, 14));

        let mut vesting = Vesting {
            total_vesting: TokenAmount::new(10_000),
            cumulative_vested: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 12,
            cliff_periods: 12,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;
        // Check that cumulative vested amount is correct
        assert_eq!(vesting.cumulative_vested, TokenAmount::new(0));

        clock = TimeStamp::new_dt(Utc.ymd(2021, 6, 15));
        vesting.update_vested_tokens(clock.time)?;
        // Check that cumulative vested amount is correct
        assert_eq!(vesting.cumulative_vested, TokenAmount::new(10_000));

        // Check that nothing else has changed
        assert_eq!(vesting.total_vesting, TokenAmount::new(10_000));
        assert_eq!(vesting.cliff_periods, 12);
        assert_eq!(vesting.total_periods, 12);
        assert_eq!(vesting.start_ts, TimeStamp::new_dt(Utc.ymd(2020, 6, 15)));

        Ok(())
    }

    #[test]
    fn it_stops_updating_vested_tokens_if_after_fully_vested() -> Result<()> {
        let clock = TimeStamp::new_dt(Utc.ymd(2024, 6, 15));

        let mut vesting = Vesting {
            total_vesting: TokenAmount::new(10_000),
            cumulative_vested: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 48,
            cliff_periods: 12,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;
        assert_eq!(vesting.cumulative_vested, TokenAmount::new(10_000));

        let clock = TimeStamp::new_dt(Utc.ymd(2030, 6, 15));
        vesting.update_vested_tokens(clock.time)?;
        assert_eq!(vesting.cumulative_vested, TokenAmount::new(10_000));

        Ok(())
    }

    #[test]
    fn it_updates_vested_tokens_monthly() -> Result<()> {
        let mut vesting = Vesting {
            total_vesting: TokenAmount::new(10_000),
            cumulative_vested: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 48,
            cliff_periods: 12,
            ..Default::default()
        };

        let mut clock;
        let mut current_month = 7;
        let mut current_year = 2020;
        for i in 1..=48 {
            clock = TimeStamp::new_dt(Utc.ymd(current_year, current_month, 20));
            vesting.update_vested_tokens(clock.time)?;

            // Check that cumulative vested amount is correct
            let vested_tokens = if i <= 11 { 0 } else { i * 10_000 / 48 };

            assert_eq!(vesting.cumulative_vested, TokenAmount::new(vested_tokens));

            // Increment month and year datetime
            current_year = if current_month == 12 {
                current_year + 1
            } else {
                current_year
            };
            current_month = if current_month < 12 {
                current_month + 1
            } else {
                1
            };
        }
        Ok(())
    }

    #[test]
    fn it_updates_vested_tokens_daily() -> Result<()> {
        let mut vesting = Vesting {
            period_type: PeriodType::Daily,
            total_vesting: TokenAmount::new(10_000),
            cumulative_vested: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 10)),
            total_periods: 100,
            cliff_periods: 25,
            ..Default::default()
        };

        let mut current_dt: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(vesting.start_ts.time, 0), Utc);

        let cliff_dt = current_dt
            .checked_add_signed(Duration::days(vesting.cliff_periods as i64))
            .unwrap();

        let mut clock;
        for i in 1..=110 {
            current_dt = current_dt.checked_add_signed(Duration::days(1)).unwrap();

            clock = TimeStamp::new_dt(current_dt.date());
            vesting.update_vested_tokens(clock.time)?;

            // Check that cumulative vested amount is correct
            let vested_tokens = if current_dt < cliff_dt {
                0
            } else if i < 100 {
                i * 10_000 / 100
            } else {
                10_000
            };

            assert_eq!(vesting.cumulative_vested, TokenAmount::new(vested_tokens));
        }
        Ok(())
    }

    #[test]
    fn it_updates_vested_tokens_when_vesting_day_is_eom() -> Result<()> {
        let clock = TimeStamp::new_dt(Utc.ymd(2022, 2, 28));

        let mut vesting = Vesting {
            total_vesting: TokenAmount::new(10_000),
            cumulative_vested: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2022, 1, 31)),
            total_periods: 48,
            cliff_periods: 0,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;

        // Check that cumulative vested amount is correct
        assert_eq!(vesting.cumulative_vested, TokenAmount::new(0));

        let clock = TimeStamp::new_dt(Utc.ymd(2022, 3, 1));
        vesting.update_vested_tokens(clock.time)?;
        assert_eq!(vesting.cumulative_vested, TokenAmount::new(208));
        Ok(())
    }

    #[test]
    fn it_updates_unfunded_liabilities_when_positive() -> Result<()> {
        let mut vesting = Vesting {
            cumulative_vested: TokenAmount::new(5_000),
            cumulative_withdrawn: TokenAmount::new(2500),
            vault_balance: TokenAmount::new(500),
            ..Default::default()
        };

        vesting.update_unfunded_liability()?;

        // Unfunded liability = 5_000 - 2_500 - 500
        assert_eq!(vesting.unfunded_liability, TokenAmount::new(2_000));

        Ok(())
    }

    #[test]
    fn it_does_not_update_unfunded_liabilities_when_none() -> Result<()> {
        let mut vesting = Vesting {
            cumulative_vested: TokenAmount::new(5_000),
            cumulative_withdrawn: TokenAmount::new(5_000),
            vault_balance: TokenAmount::new(0),
            ..Default::default()
        };

        vesting.update_unfunded_liability()?;

        assert_eq!(vesting.unfunded_liability, TokenAmount::new(0));

        Ok(())
    }

    #[test]
    fn it_does_not_update_unfunded_liabilities_when_overfunded() -> Result<()> {
        let mut vesting = Vesting {
            cumulative_vested: TokenAmount::new(5_000),
            cumulative_withdrawn: TokenAmount::new(0),
            vault_balance: TokenAmount::new(10_000),
            ..Default::default()
        };

        vesting.update_unfunded_liability()?;

        assert_eq!(vesting.unfunded_liability, TokenAmount::new(0));

        Ok(())
    }

    #[test]
    fn it_computes_delta_periods_daily() -> Result<()> {
        let mut vesting = Vesting {
            period_type: PeriodType::Daily,
            ..Default::default()
        };
        let cliff_ts = TimeStamp::new_dt(Utc.ymd(2022, 3, 1));
        let current_ts = TimeStamp::new_dt(Utc.ymd(2022, 3, 1));

        // Converting timestamps to datetimes
        let mut current_dt: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(current_ts.time, 0), Utc);

        let cliff_dt: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(cliff_ts.time, 0), Utc);

        let mut delta_days = vesting.compute_delta_periods(current_dt, cliff_dt)?;

        assert_eq!(delta_days, 0);
        for i in 1..=366 {
            current_dt = current_dt.checked_add_signed(Duration::days(1)).unwrap();

            delta_days = vesting.compute_delta_periods(current_dt, cliff_dt)?;
            assert_eq!(delta_days, i);
        }

        Ok(())
    }

    #[test]
    fn it_computes_delta_periods_monthly() -> Result<()> {
        let mut vesting = Vesting {
            period_type: PeriodType::Monthly,
            ..Default::default()
        };
        let cliff_ts = TimeStamp::new_dt(Utc.ymd(2022, 3, 1));
        let mut current_ts = TimeStamp::new_dt(Utc.ymd(2022, 3, 1));

        // Converting timestamps to datetimes
        let mut current_dt: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(current_ts.time, 0), Utc);

        let cliff_dt: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(cliff_ts.time, 0), Utc);

        let mut delta_months = vesting.compute_delta_periods(current_dt, cliff_dt)?;

        assert_eq!(delta_months, 0);

        let mut current_month = 4;
        let mut current_year = 2022;
        for i in 1..=48 {
            current_ts = TimeStamp::new_dt(Utc.ymd(current_year, current_month, 1));
            current_dt = DateTime::from_utc(NaiveDateTime::from_timestamp(current_ts.time, 0), Utc);

            delta_months = vesting.compute_delta_periods(current_dt, cliff_dt)?;

            assert_eq!(delta_months, i as u64);

            // Increment month and year datetime
            current_year = if current_month == 12 {
                current_year + 1
            } else {
                current_year
            };
            current_month = if current_month < 12 {
                current_month + 1
            } else {
                1
            };
        }

        Ok(())
    }
}
