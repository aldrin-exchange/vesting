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
}

impl From<TokenAmount> for Decimal {
    fn from(tokens: TokenAmount) -> Self {
        Decimal::from(tokens.amount)
    }
}

impl From<u64> for TokenAmount {
    fn from(amount: u64) -> Self {
        Self { amount }
    }
}
