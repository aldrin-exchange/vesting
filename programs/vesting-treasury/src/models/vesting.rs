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
    /// Cumulative amount that vested thus far
    pub cumulative_vested_amount: TokenAmount,
    /// Cumulative amount withdrawn thus far
    pub cumulative_withdrawn_amount: TokenAmount,
    /// Current amount sitting in the vesting vault
    pub vault_balance: TokenAmount,
    /// Current amount of vested tokens that is unfunded
    pub unfunded_liability: TokenAmount,
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
