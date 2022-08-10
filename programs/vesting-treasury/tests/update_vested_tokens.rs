use ::vesting_treasury::prelude::*;
use ::vesting_treasury::vesting_treasury::update_vested_tokens;
use anchor_lang::prelude::Clock;
use anchortest::{builder::*, stub};
use serial_test::serial;
use solana_sdk::clock::{Epoch, UnixTimestamp};
use solana_sdk::instruction::Instruction;
use std::sync::{Arc, Mutex};

#[test]
#[serial]
fn update_monthly_vested_tokens() -> Result<()> {
    let vesting_before = Vesting {
        total_vesting: TokenAmount::new(10_000),
        cumulative_vested: TokenAmount::new(0),
        start_ts: TimeStamp::new_dt(Utc.ymd(2020, 1, 1)),
        total_periods: 48,
        cliff_periods: 12,
        ..Default::default()
    };

    let mut test = Tester::new(vesting_before.clone(), 500);

    let mut current_clock = TimeStamp::new_dt(Utc.ymd(2020, 1, 1));
    test.update_vested_tokens(current_clock.time)?;
    let mut vesting_after = test.vesting_copy();

    assert_eq!(vesting_after.cumulative_vested.amount, 0);
    assert_eq!(vesting_after.unfunded_liability.amount, 0);

    current_clock = TimeStamp::new_dt(Utc.ymd(2021, 1, 1));
    test.update_vested_tokens(current_clock.time)?;
    vesting_after = test.vesting_copy();

    assert_eq!(vesting_after.cumulative_vested.amount, 2_500);
    assert_eq!(vesting_after.unfunded_liability.amount, 2_500);

    current_clock = TimeStamp::new_dt(Utc.ymd(2022, 1, 1));
    test.update_vested_tokens(current_clock.time)?;
    vesting_after = test.vesting_copy();

    assert_eq!(vesting_after.cumulative_vested.amount, 5_000);
    assert_eq!(vesting_after.unfunded_liability.amount, 5_000);

    current_clock = TimeStamp::new_dt(Utc.ymd(2023, 1, 1));
    test.update_vested_tokens(current_clock.time)?;
    vesting_after = test.vesting_copy();

    assert_eq!(vesting_after.cumulative_vested.amount, 7_500);
    assert_eq!(vesting_after.unfunded_liability.amount, 7_500);

    current_clock = TimeStamp::new_dt(Utc.ymd(2024, 1, 1));
    test.update_vested_tokens(current_clock.time)?;
    vesting_after = test.vesting_copy();

    assert_eq!(vesting_after.cumulative_vested.amount, 10_000);
    assert_eq!(vesting_after.unfunded_liability.amount, 10_000);

    current_clock = TimeStamp::new_dt(Utc.ymd(2050, 1, 1));
    test.update_vested_tokens(current_clock.time)?;
    vesting_after = test.vesting_copy();

    assert_eq!(vesting_after.cumulative_vested.amount, 10_000);
    assert_eq!(vesting_after.unfunded_liability.amount, 10_000);

    Ok(())
}

// #[test]
// #[serial]
// fn update_daily_vested_tokens() -> Result<()> {
//     let vesting_before = Vesting {
//         total_vesting: TokenAmount::new(10_000),
//         cumulative_vested: TokenAmount::new(0),
//         start_ts: TimeStamp::new_dt(Utc.ymd(2020, 1, 1)),
//         total_periods: 48,
//         cliff_periods: 12,
//         ..Default::default()
//     };

//     let mut test = Tester::new(vesting_before.clone(), 500);

//     let mut current_clock = TimeStamp::new_dt(Utc.ymd(2020, 1, 1));
//     test.update_vested_tokens(current_clock.time)?;
//     let mut vesting_after = test.vesting_copy();

//     assert_eq!(vesting_after.cumulative_vested.amount, 0);
//     assert_eq!(vesting_after.unfunded_liability.amount, 0);

//     current_clock = TimeStamp::new_dt(Utc.ymd(2021, 1, 1));
//     test.update_vested_tokens(current_clock.time)?;
//     vesting_after = test.vesting_copy();

//     assert_eq!(vesting_after.cumulative_vested.amount, 2_500);
//     assert_eq!(vesting_after.unfunded_liability.amount, 2_500);

//     current_clock = TimeStamp::new_dt(Utc.ymd(2022, 1, 1));
//     test.update_vested_tokens(current_clock.time)?;
//     vesting_after = test.vesting_copy();

//     assert_eq!(vesting_after.cumulative_vested.amount, 5_000);
//     assert_eq!(vesting_after.unfunded_liability.amount, 5_000);

//     current_clock = TimeStamp::new_dt(Utc.ymd(2023, 1, 1));
//     test.update_vested_tokens(current_clock.time)?;
//     vesting_after = test.vesting_copy();

//     assert_eq!(vesting_after.cumulative_vested.amount, 7_500);
//     assert_eq!(vesting_after.unfunded_liability.amount, 7_500);

//     current_clock = TimeStamp::new_dt(Utc.ymd(2024, 1, 1));
//     test.update_vested_tokens(current_clock.time)?;
//     vesting_after = test.vesting_copy();

//     assert_eq!(vesting_after.cumulative_vested.amount, 10_000);
//     assert_eq!(vesting_after.unfunded_liability.amount, 10_000);

//     current_clock = TimeStamp::new_dt(Utc.ymd(2050, 1, 1));
//     test.update_vested_tokens(current_clock.time)?;
//     vesting_after = test.vesting_copy();

//     assert_eq!(vesting_after.cumulative_vested.amount, 10_000);
//     assert_eq!(vesting_after.unfunded_liability.amount, 10_000);

//     Ok(())
// }

#[derive(Clone, Debug, PartialEq)]
struct Tester {
    vesting: AccountInfoWrapper,
    slot: u64,
}

impl Default for Tester {
    fn default() -> Self {
        let vesting = AccountInfoWrapper::new()
            .signer()
            .owner(vesting_treasury::ID)
            .mutable()
            .size(Vesting::space());

        Self { vesting, slot: 0 }
    }
}

impl Tester {
    fn new(vesting_data: Vesting, slot: u64) -> Self {
        let vesting = AccountInfoWrapper::new()
            .owner(vesting_treasury::ID)
            .mutable()
            .data(vesting_data.clone());

        Self { vesting, slot }
    }

    fn vesting_copy(&self) -> Vesting {
        Vesting::try_deserialize(&mut self.vesting.data.as_slice()).unwrap()
    }

    fn update_vested_tokens(&mut self, current_ts: i64) -> Result<()> {
        self.set_syscalls(
            CpiValidatorState::UpdateVestedTokens {
                vesting: self.vesting.key,
            },
            current_ts,
        );
        let mut ctx = self.context_wrapper();
        let mut accounts = ctx.accounts()?;

        update_vested_tokens(ctx.build(&mut accounts))?;
        accounts.exit(&vesting_treasury::ID)?;

        Ok(())
    }

    fn context_wrapper(&mut self) -> ContextWrapper {
        ContextWrapper::new(vesting_treasury::ID).acc(&mut self.vesting)
    }

    fn set_syscalls(&self, state: CpiValidatorState, clock: i64) -> Arc<Mutex<CpiValidatorState>> {
        let state = Arc::new(Mutex::new(state));

        let syscalls = stub::Syscalls::new(CpiValidator(Arc::clone(&state)));
        let clock = Clock {
            slot: 10,
            epoch_start_timestamp: clock as UnixTimestamp,
            epoch: 10 as Epoch,
            leader_schedule_epoch: 10 as Epoch,
            unix_timestamp: clock as UnixTimestamp,
        };

        syscalls.clock(clock);
        syscalls.set();

        state
    }
}

struct CpiValidator(Arc<Mutex<CpiValidatorState>>);
#[derive(Debug, Eq, PartialEq)]
enum CpiValidatorState {
    UpdateVestedTokens { vesting: Pubkey },
    Done,
}

impl stub::ValidateCpis for CpiValidator {
    fn validate_next_instruction(&mut self, ix: &Instruction, accounts: &[AccountInfo]) {
        let mut state = self.0.lock().unwrap();
        match *state {
            CpiValidatorState::UpdateVestedTokens { vesting } => {
                *state = CpiValidatorState::Done;
            }
            CpiValidatorState::Done => {
                panic!("No more instructions expected, got {:#?}", ix);
            }
        }
    }
}
