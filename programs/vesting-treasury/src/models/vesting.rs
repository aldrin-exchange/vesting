use crate::prelude::*;
use std::mem;

#[derive(Default, Debug)]
#[account]
pub struct Vesting {
    /// The vestee of this Vesting account.
    pub beneficiary: Pubkey,
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
    pub start_ts: i64,
    /// The time in Unix Timestamp at which all tokens are vested.
    pub end_ts: i64,
    /// The time at which the cliff period ends, if any.
    pub cliff_end_ts: i64,
    /// The number of times vesting will occur. For example, if vesting
    /// is once a year over seven years, this will be 7. This excludes the
    /// the cliff period.
    pub period_count: u64,
}

impl Vesting {
    pub const VAULT_PREFIX: &'static [u8; 5] = b"vault";
    pub const SIGNER_PDA_PREFIX: &'static [u8; 6] = b"signer";

    pub fn space() -> usize {
        let discriminant = 8;
        let beneficiary = 32;
        let mint = 32;
        let vault = 32;

        let total_vesting_amount = mem::size_of::<TokenAmount>();
        let cumulative_vested_amount = mem::size_of::<TokenAmount>();
        let cumulative_withdrawn_amount = mem::size_of::<TokenAmount>();
        let vesting_vault_balance = mem::size_of::<TokenAmount>();
        let unfunded_liabilities = mem::size_of::<TokenAmount>();

        let start_ts = mem::size_of::<i32>();
        let end_ts = mem::size_of::<i32>();
        let cliff_end_ts = mem::size_of::<i32>();
        let period_count = mem::size_of::<i32>();

        let result = discriminant
            + beneficiary
            + mint
            + vault
            + mint
            + total_vesting_amount
            + cumulative_vested_amount
            + cumulative_withdrawn_amount
            + vesting_vault_balance
            + unfunded_liabilities
            + start_ts
            + end_ts
            + cliff_end_ts
            + period_count;

        // panic!("Result is {}", result);

        discriminant
            + beneficiary
            + mint
            + vault
            + mint
            + total_vesting_amount
            + cumulative_vested_amount
            + cumulative_withdrawn_amount
            + vesting_vault_balance
            + unfunded_liabilities
            + start_ts
            + end_ts
            + cliff_end_ts
            + period_count
    }
}
