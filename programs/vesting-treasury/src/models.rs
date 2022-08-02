pub mod vesting;

use crate::prelude::*;
pub use vesting::*;

#[derive(
    AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd,
)]
pub struct TokenAmount {
    pub amount: u64,
}

#[derive(
    AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd,
)]
pub struct TimeStamp {
    pub time: i64,
}

#[derive(
    AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd,
)]
pub struct Slot {
    pub slot: u64,
}

#[derive(
    AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd,
)]
pub struct Permillion {
    /// 1% = 10_000
    pub permillion: u64,
}

impl TokenAmount {
    pub fn new(amount: u64) -> Self {
        Self { amount }
    }

    pub fn max_value() -> Self {
        Self {
            amount: std::u64::MAX,
        }
    }
}

impl TimeStamp {
    pub fn new(time: i64) -> Self {
        Self { time }
    }
    pub fn new_dt(date: Date<Utc>) -> Self {
        let time = date.and_hms_nano(0, 0, 0, 0).timestamp();

        Self { time }
    }
    pub fn current() -> Result<Self> {
        Ok(Self {
            time: Clock::get()?.unix_timestamp,
        })
    }
}

impl Slot {
    pub fn new(slot: u64) -> Self {
        Self { slot }
    }

    pub fn current() -> Result<Self> {
        Ok(Self {
            slot: Clock::get()?.slot,
        })
    }
}

impl Permillion {
    pub fn from_percent(percent: u64) -> Self {
        Self {
            permillion: percent.checked_mul(10_000).unwrap(),
        }
    }
}

impl From<TokenAmount> for Decimal {
    fn from(tokens: TokenAmount) -> Self {
        Decimal::from(tokens.amount)
    }
}

impl From<Permillion> for Decimal {
    fn from(permillion: Permillion) -> Self {
        Decimal::from_permillion(permillion.permillion)
    }
}

impl From<u64> for TokenAmount {
    fn from(amount: u64) -> Self {
        Self { amount }
    }
}
