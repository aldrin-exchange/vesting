use crate::prelude::*;
use std::mem;

#[derive(Default, Debug)]
#[account]
pub struct Vesting {
    /// The authority of this Vesting account.
    pub admin: Pubkey,
    /// The vestee of this Vesting account.
    pub vestee_wallet: Pubkey,
    /// The mint of the SPL token locked up.
    pub mint: Pubkey,
    /// Address of the account's token vault.
    pub vault: Pubkey,
    /// The total amount that will vest over time
    pub total_vesting_amount: TokenAmount,
    /// Amount that is vested thus far
    pub cumulative_vested_amount: TokenAmount,
    /// Amount that has been withdrawn thus far
    pub cumulative_withdrawn_amount: TokenAmount,
    /// Current amount sitting in the vesting vault
    pub vesting_vault_balance: TokenAmount,
    /// Current amount of vested tokens that is unfunded
    pub unfunded_liabilities: TokenAmount,
    /// The start time in Unix Timestamp of the vesting period
    pub start_ts: TimeStamp,
    /// The amount of periods in total in the vesting schedule
    pub total_periods: u64,
    /// The amount of periods in the cliff part of the schedule
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

        let total_vesting_amount = mem::size_of::<TokenAmount>();
        let cumulative_vested_amount = mem::size_of::<TokenAmount>();
        let cumulative_withdrawn_amount = mem::size_of::<TokenAmount>();
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
            + total_vesting_amount
            + cumulative_vested_amount
            + cumulative_withdrawn_amount
            + vesting_vault_balance
            + unfunded_liabilities
            + start_ts
            + total_periods
            + cliff_periods
            + period_type
    }

    pub fn update_vested_tokens(&mut self, clock_ts: i64) -> Result<()> {
        // 1. Convert clock to datetime
        let current_dt: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(clock_ts, 0), Utc);

        // 2. Convert start_at to datetime
        let start_dt: DateTime<Utc> =
            DateTime::from_utc(NaiveDateTime::from_timestamp(self.start_ts.time, 0), Utc);

        // 3. Add cliff periods to start_at to get cliff_date
        let cliff_dt = shift_months(start_dt, self.cliff_periods as i32);

        //
        let end_dt = shift_months(start_dt, self.total_periods as i32);

        if current_dt < cliff_dt {
            // We are still in the cliff period and therefore we
            // do not have to update the vested tokens because there are none
            return Ok(());
        }

        if current_dt >= end_dt {
            // This means the schedule is fully vested
            self.cumulative_vested_amount = self.total_vesting_amount;
            return Ok(());
        }

        // 4. Compute difference Δyear = year(clock) - year(cliff)
        let delta_years = (current_dt.year() - cliff_dt.year()) as u32;

        // 5. Match Δyear:
        let delta_periods = match delta_years {
            // Δyear == 0 => compute_periods_from_cliff
            0 => compute_periods_from_cliff_to_current_dt(cliff_dt, current_dt),
            // Δyear == 1 => compute_periods_until_eoy + compute_periods_from_boy
            1 => {
                compute_periods_from_cliff_to_eoy(cliff_dt)
                    + compute_periods_from_boy_to_current_dt(cliff_dt, current_dt)
            }
            // year > 2 => compute_periods_until_eoy
            // + compute_periods_in_full_years + compute_periods_from_boy
            _ => {
                compute_periods_from_cliff_to_eoy(cliff_dt)
                    + compute_periods_in_full_years(delta_years)
                    + compute_periods_from_boy_to_current_dt(cliff_dt, current_dt)
            }
        };

        // 6. Compute cumulative_vested = (cliff_periods + Δperiods)
        //                                   * total_amount / total_periods
        let cumulative_vested = Decimal::from(self.cliff_periods)
            .try_add(Decimal::from(delta_periods as u64))?
            .try_div(Decimal::from(self.total_periods))?
            .try_mul(Decimal::from(self.total_vesting_amount))?
            .try_floor()?;

        self.cumulative_vested_amount = TokenAmount::new(cumulative_vested);

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum PeriodType {
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
            1 => Ok(PeriodType::Monthly),
            2 => Ok(PeriodType::Quarterly),
            3 => Ok(PeriodType::SemiAnnually),
            4 => Ok(PeriodType::Yearly),
            _ => Err(error!(err::arg("The period type enumeration is invalid"))),
        }
    }
}

pub fn compute_periods_from_cliff_to_current_dt(
    cliff_dt: DateTime<Utc>,
    current_dt: DateTime<Utc>,
) -> u32 {
    if current_dt.month() < current_dt.month() {
        return 0;
    }

    let delta_periods = current_dt.month()
        - cliff_dt.month()
        - if current_dt.day() < cliff_dt.day() {
            1
        } else {
            0
        };
    delta_periods
}

pub fn compute_periods_from_cliff_to_eoy(cliff_dt: DateTime<Utc>) -> u32 {
    12 - cliff_dt.month()
}

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

pub fn compute_periods_in_full_years(delta_years: u32) -> u32 {
    (delta_years - 1) * 12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_update_vested_tokens_if_before_cliff_date() -> Result<()> {
        let clock = TimeStamp::new_dt(Utc.ymd(2021, 6, 14));

        let mut vesting = Vesting {
            total_vesting_amount: TokenAmount::new(10_000),
            cumulative_vested_amount: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 48,
            cliff_periods: 12,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;

        // Check that nothing has changed
        assert_eq!(vesting.cumulative_vested_amount, TokenAmount::new(0));
        assert_eq!(vesting.total_vesting_amount, TokenAmount::new(10_000));
        assert_eq!(vesting.cliff_periods, 12);
        assert_eq!(vesting.total_periods, 48);
        assert_eq!(vesting.start_ts, TimeStamp::new_dt(Utc.ymd(2020, 6, 15)));

        Ok(())
    }

    #[test]
    fn it_updates_vested_tokens_if_is_cliff_date() -> Result<()> {
        let clock = TimeStamp::new_dt(Utc.ymd(2021, 6, 15));

        let mut vesting = Vesting {
            total_vesting_amount: TokenAmount::new(10_000),
            cumulative_vested_amount: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 48,
            cliff_periods: 12,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;

        // Check that cumulative vested amount is correct
        assert_eq!(vesting.cumulative_vested_amount, TokenAmount::new(2_500));

        // Check that nothing else has changed
        assert_eq!(vesting.total_vesting_amount, TokenAmount::new(10_000));
        assert_eq!(vesting.cliff_periods, 12);
        assert_eq!(vesting.total_periods, 48);
        assert_eq!(vesting.start_ts, TimeStamp::new_dt(Utc.ymd(2020, 6, 15)));

        Ok(())
    }

    #[test]
    fn it_updates_vested_tokens_if_after_cliff_date() -> Result<()> {
        let clock = TimeStamp::new_dt(Utc.ymd(2022, 6, 15));

        let mut vesting = Vesting {
            total_vesting_amount: TokenAmount::new(10_000),
            cumulative_vested_amount: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 48,
            cliff_periods: 12,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;

        // Check that cumulative vested amount is correct
        assert_eq!(vesting.cumulative_vested_amount, TokenAmount::new(5_000));

        // Check that nothing else has changed
        assert_eq!(vesting.total_vesting_amount, TokenAmount::new(10_000));
        assert_eq!(vesting.cliff_periods, 12);
        assert_eq!(vesting.total_periods, 48);
        assert_eq!(vesting.start_ts, TimeStamp::new_dt(Utc.ymd(2020, 6, 15)));

        Ok(())
    }

    #[test]
    fn it_stops_updating_vested_tokens_if_after_fully_vested() -> Result<()> {
        let clock = TimeStamp::new_dt(Utc.ymd(2024, 6, 15));

        let mut vesting = Vesting {
            total_vesting_amount: TokenAmount::new(10_000),
            cumulative_vested_amount: TokenAmount::new(0),
            start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
            total_periods: 48,
            cliff_periods: 12,
            ..Default::default()
        };

        vesting.update_vested_tokens(clock.time)?;
        assert_eq!(vesting.cumulative_vested_amount, TokenAmount::new(10_000));

        let clock = TimeStamp::new_dt(Utc.ymd(2030, 6, 15));
        vesting.update_vested_tokens(clock.time)?;
        assert_eq!(vesting.cumulative_vested_amount, TokenAmount::new(10_000));

        Ok(())
    }

    #[test]
    fn it_updates_vested_tokens() -> Result<()> {
        let mut vesting = Vesting {
            total_vesting_amount: TokenAmount::new(10_000),
            cumulative_vested_amount: TokenAmount::new(0),
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

            assert_eq!(
                vesting.cumulative_vested_amount,
                TokenAmount::new(vested_tokens)
            );

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
