use ::vesting_treasury::prelude::*;
use ::vesting_treasury::vesting_treasury::update_vested_tokens;
// use anchor_lang::solana_program::system_instruction;
use anchortest::{builder::*, stub};
use serial_test::serial;
use solana_sdk::instruction::Instruction;
use std::sync::{Arc, Mutex};

#[test]
#[serial]
fn swaps_const_prod_two_reserves_no_discount() -> Result<()> {
    let vesting_before = Vesting {
        total_vesting: TokenAmount::new(10_000),
        cumulative_vested: TokenAmount::new(0),
        start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
        total_periods: 48,
        cliff_periods: 12,
        ..Default::default()
    };

    let mut test = Tester::new(vesting_before.clone(), 500);

    test.update_vested_tokens()?;

    let _vesting_after = test.vesting_copy();

    Ok(())
}

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

    fn update_vested_tokens(&mut self) -> Result<()> {
        let mut ctx = self.context_wrapper();
        let mut accounts = ctx.accounts()?;

        update_vested_tokens(ctx.build(&mut accounts))?;
        accounts.exit(&vesting_treasury::ID)?;

        Ok(())
    }

    fn context_wrapper(&mut self) -> ContextWrapper {
        ContextWrapper::new(vesting_treasury::ID).acc(&mut self.vesting)
    }

    fn set_syscalls(&self, state: CpiValidatorState) -> Arc<Mutex<CpiValidatorState>> {
        let state = Arc::new(Mutex::new(state));

        let syscalls = stub::Syscalls::new(CpiValidator(Arc::clone(&state)));

        syscalls.slot(self.slot);
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
