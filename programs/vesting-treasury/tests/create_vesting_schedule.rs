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
use solana_sdk::instruction::Instruction;
use solana_sdk::sysvar::rent;

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
        let vesting_vault = AccountInfoWrapper::new()
            .pack(spl::token_account::new(vesting_signer.key))
            .owner(token::ID);
        let mint = AccountInfoWrapper::new()
            .pack(spl::mint::new(vesting_signer.key))
            .owner(token::ID);

        let vestee_wallet = AccountInfoWrapper::new()
            .pack(spl::token_account::new(admin.key))
            .owner(token::ID);
        let token_program = AccountInfoWrapper::with_key(token::ID).program();
        let system_program = AccountInfoWrapper::with_key(system_program::ID).program();
        let rent = AccountInfoWrapper::with_key(rent::ID).program();

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
        self.set_syscalls();

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

    fn set_syscalls(&self) {
        stub::Syscalls::new(CpiValidator(CpiValidatorState::CreateVesting {
            admin: self.admin.key,
            vesting: self.vesting.key,
        }))
        .set();
    }
}

struct CpiValidator(CpiValidatorState);
enum CpiValidatorState {
    CreateVesting { admin: Pubkey, vesting: Pubkey },
    Done,
}

impl stub::ValidateCpis for CpiValidator {
    fn validate_next_instruction(&mut self, ix: &Instruction, accounts: &[AccountInfo]) {
        match self.0 {
            CpiValidatorState::CreateVesting { admin, vesting } => {
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

                self.0 = CpiValidatorState::Done;
            }
            CpiValidatorState::Done => {
                panic!("No more instructions expected, got {:#?}", ix);
            }
        }
    }
}
