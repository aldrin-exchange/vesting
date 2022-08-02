use ::vesting_treasury::prelude::*;
use ::vesting_treasury::vesting_treasury::update_vested_tokens;
use anchortest::builder::*;
use serial_test::serial;

#[test]
#[serial]
fn swaps_const_prod_two_reserves_no_discount() -> Result<()> {
    let vesting_before = Vesting {
        total_vesting_amount: TokenAmount::new(10_000),
        cumulative_vested_amount: TokenAmount::new(0),
        start_ts: TimeStamp::new_dt(Utc.ymd(2020, 6, 15)),
        total_periods: 48,
        cliff_periods: 12,
        ..Default::default()
    };

    let mut test = Tester::new(vesting_before.clone());

    test.update_vested_tokens()?;

    let _vesting_after = test.vesting_copy();

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
struct Tester {
    vesting: AccountInfoWrapper,
}

impl Default for Tester {
    fn default() -> Self {
        let vesting = AccountInfoWrapper::new()
            .signer()
            .owner(vesting_treasury::ID)
            .mutable()
            .size(Vesting::space());

        Self { vesting }
    }
}

impl Tester {
    fn new(vesting_data: Vesting) -> Self {
        let vesting = AccountInfoWrapper::new()
            .owner(vesting_treasury::ID)
            .mutable()
            .data(vesting_data.clone());

        Self { vesting }
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
}
