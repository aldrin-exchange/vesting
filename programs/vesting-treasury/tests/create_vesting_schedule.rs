use ::vesting_treasury::prelude::*;
use ::vesting_treasury::vesting_treasury::create_vesting_schedule;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::system_program;
use anchor_spl::token;
use anchortest::{
    builder::*,
    spl::{self, MintExt, TokenAccountExt},
    stub,
};
use serial_test::serial;
use solana_sdk::instruction::Instruction;
use solana_sdk::sysvar::rent;
use std::str;
use std::sync::{Arc, Mutex};

#[test]
#[serial]
fn monthly_vesting_with_cliff() -> Result<()> {
    let mut test = Tester::default();
    let og_state = test.clone();

    let vesting_amount = TokenAmount::new(10_000);
    let start_ts = TimeStamp::new(1577836801); // Jan 01 2020
    let cliff_periods = 12;
    let total_periods = 48;
    let period_type = 2; // Monthly

    assert!(test
        .create_vesting(
            vesting_amount,
            start_ts,
            cliff_periods,
            total_periods,
            period_type
        )
        .is_ok());

    let vesting = Vesting::try_deserialize(&mut test.vesting.data.as_slice())?;
    // assert_eq!(vesting.admin, test.admin.key);
    // assert_eq!(vesting.vestee_wallet, test.vestee_wallet.key);
    // assert_eq!(vesting.mint, test.mint.key);
    // assert_eq!(vesting.vault, test.vesting_vault.key);
    // assert_eq!(vesting.signer, test.vesting_signer.key); // TODO

    // assert_eq!(vesting.total_vesting_amount, vesting_amount);
    // assert_eq!(vesting.cumulative_vested_amount, TokenAmount::new(0));
    // assert_eq!(vesting.cumulative_withdrawn_amount, TokenAmount::new(0));
    // assert_eq!(vesting.vault_balance, TokenAmount::new(0));
    // assert_eq!(vesting.unfunded_liability, TokenAmount::new(0));
    // assert_eq!(vesting.start_ts, start_ts);
    // assert_eq!(vesting.total_periods, total_periods);
    // assert_eq!(vesting.cliff_periods, cliff_periods);
    // assert_eq!(vesting.period_type, PeriodType::Monthly);

    // no other changes should have happened
    // test.vesting = og_state.vesting.clone();
    // assert_eq!(test, og_state);

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
struct Tester {
    admin: AccountInfoWrapper,
    vesting: AccountInfoWrapper,
    vesting_signer: AccountInfoWrapper,
    vesting_vault: AccountInfoWrapper,
    mint: AccountInfoWrapper,
    vestee_wallet: AccountInfoWrapper,
    token_program: AccountInfoWrapper,
    system_program: AccountInfoWrapper,
    rent: AccountInfoWrapper,
}

impl Default for Tester {
    fn default() -> Self {
        let admin = AccountInfoWrapper::new().mutable().signer();
        let vesting = AccountInfoWrapper::new()
            .signer()
            .owner(vesting_treasury::ID)
            .mutable()
            .size(Vesting::space());
        let vesting_signer = AccountInfoWrapper::pda(
            vesting_treasury::ID,
            "vesting_signer",
            &[Vesting::SIGNER_PDA_PREFIX, vesting.key.as_ref()],
        );
        // TODO: for sure this should be metioning the seeds
        // seeds = [Vesting::VAULT_PREFIX, vesting.key().as_ref()],
        let mint = AccountInfoWrapper::with_key(Pubkey::new_unique())
            .mutable()
            .pack(spl::mint::new(vesting_signer.key))
            .owner(token::ID);

        let vesting_vault = AccountInfoWrapper::with_key(
            Pubkey::find_program_address(
                &[Vesting::VAULT_PREFIX, vesting.key.as_ref()],
                &vesting_treasury::ID,
            )
            .0,
        )
        .pack(spl::token_account::new(vesting_signer.key).mint(mint.key))
        .owner(token::ID);

        let vestee_wallet = AccountInfoWrapper::new()
            .pack(spl::token_account::new(admin.key))
            .owner(token::ID);
        let token_program = AccountInfoWrapper::with_key(token::ID).program();
        let system_program = AccountInfoWrapper::with_key(system_program::ID).program();
        let rent = AccountInfoWrapper::with_key(rent::ID).program();

        // println!("admin KEYYYYY::::: => {:?}", rent::ID);

        Self {
            admin,
            vesting,
            vesting_signer,
            vesting_vault,
            mint,
            vestee_wallet,
            token_program,
            system_program,
            rent,
        }
    }
}

impl Tester {
    fn create_vesting(
        &mut self,
        vesting_amount: TokenAmount,
        start_ts: TimeStamp,
        cliff_periods: u64,
        total_periods: u64,
        period_type: u32,
    ) -> Result<()> {
        self.set_syscalls(CpiValidatorState::CreateVesting {
            admin: self.admin.key,
            vesting: self.vesting.key,
            next_cpi: CreateVault {
                admin: self.admin.key,
                vesting_vault: self.vesting_vault.key,
                vesting_signer: self.vesting_signer.key,
                vesting: self.vesting.key,
                mint: self.mint.key,
            },
        });
        let mut ctx = self.context_wrapper();
        let mut accounts = ctx.accounts()?;

        create_vesting_schedule(
            ctx.build(&mut accounts),
            vesting_amount,
            start_ts,
            cliff_periods,
            total_periods,
            period_type,
        )?;
        accounts.exit(&vesting_treasury::ID)?;

        Ok(())
    }

    fn context_wrapper(&mut self) -> ContextWrapper {
        ContextWrapper::new(vesting_treasury::ID)
            .acc(&mut self.admin)
            .acc(&mut self.vesting)
            .acc(&mut self.vesting_signer)
            .acc(&mut self.vesting_vault)
            .acc(&mut self.mint)
            .acc(&mut self.vestee_wallet)
            .acc(&mut self.token_program)
            .acc(&mut self.system_program)
            .acc(&mut self.rent)
    }

    fn set_syscalls(&self, state: CpiValidatorState) -> stub::Syscalls<CpiValidator> {
        let validator = CpiValidator(Arc::new(Mutex::new(state)));

        let syscalls = stub::Syscalls::new(validator);
        syscalls.clone().set();

        syscalls
        // stub::Syscalls::new(CpiValidator(CpiValidatorState::CreateVesting {
        //     admin: self.admin.key,
        //     vesting: self.vesting.key,
        //     next_cpi: CreateVault {
        //         admin: self.admin.key,
        //         vesting_vault: self.vesting_vault.key,
        //         vesting_signer: self.vesting_signer.key,
        //     },
        // }))
        // .set();
    }
}

#[derive(Clone, Debug)]
struct CpiValidator(Arc<Mutex<CpiValidatorState>>);

#[derive(Debug, PartialEq, Eq)]
enum CpiValidatorState {
    CreateVesting {
        admin: Pubkey,
        vesting: Pubkey,
        next_cpi: CreateVault,
    },
    CreateVestingVault(CreateVault),
    Done,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CreateVault {
    admin: Pubkey,
    vesting_vault: Pubkey,
    vesting_signer: Pubkey,
    vesting: Pubkey,
    mint: Pubkey,
}

impl stub::ValidateCpis for CpiValidator {
    fn validate_next_instruction(&mut self, ix: &Instruction, accounts: &[AccountInfo]) {
        let mut state = self.0.lock().unwrap();

        match *state {
            CpiValidatorState::CreateVesting {
                admin,
                vesting,
                ref next_cpi,
            } => {
                let rent = Rent::default().minimum_balance(Vesting::space());
                let expected_ix = system_instruction::create_account(
                    &admin,
                    &vesting,
                    rent,
                    Vesting::space() as u64,
                    &vesting_treasury::ID,
                );
                assert_eq!(&expected_ix, ix);

                let vesting = accounts.iter().find(|acc| acc.key() == vesting).unwrap();
                let mut lamports = vesting.lamports.borrow_mut();
                **lamports = rent;

                *state = CpiValidatorState::CreateVestingVault(next_cpi.clone());
            }
            CpiValidatorState::CreateVestingVault(CreateVault {
                admin,
                vesting_vault,
                vesting_signer, // TODO: maybr remove?
                vesting,
                mint,
            }) => {
                let rent = Rent::default().minimum_balance(Vesting::space());
                // let expected_ix = token::spl_token::instruction::initialize_account(
                //     &token::ID,
                //     &Pubkey::find_program_address(
                //         &[Vesting::VAULT_PREFIX, vesting.as_ref()],
                //         &vesting_treasury::ID,
                //     )
                //     .0,
                //     &mint,
                //     &vesting_signer,
                // )
                // .unwrap();

                let expected_ix = system_instruction::create_account(
                    &admin,
                    &Pubkey::find_program_address(
                        &[Vesting::VAULT_PREFIX, vesting.as_ref()],
                        &vesting_treasury::ID,
                    )
                    .0,
                    rent,
                    token::TokenAccount::LEN as u64,
                    &token::ID,
                );

                // let expected_ix = system_instruction::create_account_with_seed(
                //     &admin,
                //     &vesting_vault,
                //     &vesting,
                //     &str::from_utf8(Vesting::VAULT_PREFIX).unwrap(),
                //     rent,
                //     token::TokenAccount::LEN as u64,
                //     &token::ID,
                // );

                assert_eq!(&expected_ix, ix);

                let vesting_vault = accounts
                    .iter()
                    .find(|acc| acc.key() == vesting_vault)
                    .unwrap();
                let mut lamports = vesting_vault.lamports.borrow_mut();
                **lamports = rent;

                *state = CpiValidatorState::Done;
            }
            CpiValidatorState::Done => {
                panic!("No more instructions expected, got {:#?}", ix);
            }
        }
    }
}
