use crate::prelude::*;
use std::fmt::Display;

#[error_code]
pub enum TreasuryError {
    /// Use this error via the [`acc`] function to provide more background
    /// about the issue.
    #[msg("Provided account breaks some constraints, see logs for more info")]
    InvalidAccountInput,
    /// Use this error via the [`arg`] function to provide more background
    /// about the issue.
    #[msg("One of the provided input arguments is invalid")]
    InvalidArg,
    #[msg("Vesting admin does not match the provided signer")]
    VestingAdminMismatch,
    /// Use this error for program paths which should never be reached if the
    /// program logic works as intended.
    #[msg("There's a bug in the program, see logs for more info")]
    InvariantViolation,
}

pub fn acc(msg: impl Display) -> TreasuryError {
    msg!("[InvalidAccountInput] {}", msg);

    TreasuryError::InvalidAccountInput
}

pub fn arg(msg: impl Display) -> TreasuryError {
    msg!("[InvalidArg] {}", msg);

    TreasuryError::InvalidArg
}
