//! Initializes new [`Vesting`] account. After this call,
//! the admin can fund the vesting vault such that the tokens
//! become available to the beneficiary as they vest over time.

use crate::prelude::*;

use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreateVestingSchedule<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(zero)]
    pub vesting: Account<'info, Vesting>,
    /// CHECK: UNSAFE_CODES.md#signer
    #[account(
        seeds = [Vesting::SIGNER_PDA_PREFIX, vesting.key().as_ref()],
        bump
    )]
    pub vesting_signer: AccountInfo<'info>,
    /// CHECK: UNSAFE_CODES.md#token
    #[account(
        init,
        payer = admin,
        space = TokenAccount::LEN,
        owner = token_program.key(),
        seeds = [Vesting::VAULT_PREFIX, vesting.key().as_ref()],
        bump,
    )]
    pub vesting_vault: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        constraint = vestee_wallet.mint == mint.key()
        @ err::acc("Vestee wallet must be of correct mint")
    )]
    pub vestee_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /// CHECK: UNSAFE_CODES.md#token
    pub rent: AccountInfo<'info>,
}

pub fn handle(
    ctx: Context<CreateVestingSchedule>,
    vesting_amount: TokenAmount,
    start_ts: TimeStamp,
    cliff_periods: u64,
    total_periods: u64,
    // TODO: Add this to the documentation explaining
    // the mapping from u32 to PeriodType Enum
    period_type: u32,
) -> Result<()> {
    if period_type != 1 {
        return Err(error!(err::arg(
            "The current contract version only supports\
             vesting schedules with monthly periods."
        )));
    }

    if cliff_periods > total_periods {
        return Err(error!(err::arg(
            "The number of cliff periods cannot be higher than total number of periods"
        )));
    }

    let vesting_signer_bump_seed = *ctx.bumps.get("vesting_signer").unwrap();

    let accs = ctx.accounts;

    accs.vesting.admin = accs.admin.key();
    accs.vesting.vestee_wallet = accs.vestee_wallet.key();
    accs.vesting.mint = accs.mint.key();
    accs.vesting.vault = accs.vesting_vault.key();

    accs.vesting.total_vesting_amount = vesting_amount;

    accs.vesting.start_ts = start_ts;
    accs.vesting.total_periods = total_periods;
    accs.vesting.cliff_periods = cliff_periods;
    accs.vesting.period_type = PeriodType::from_u32(period_type)?;

    msg!("Initializing vesting vault");

    let signer_seed = &[
        Vesting::SIGNER_PDA_PREFIX,
        &accs.vesting.key().to_bytes()[..],
        &[vesting_signer_bump_seed],
    ];
    token::initialize_account(
        accs.as_init_vesting_vault_context()
            .with_signer(&[&signer_seed[..]]),
    )?;

    Ok(())
}

impl<'info> CreateVestingSchedule<'info> {
    pub fn as_init_vesting_vault_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::InitializeAccount<'info>> {
        let cpi_accounts = token::InitializeAccount {
            mint: self.mint.to_account_info(),
            authority: self.vesting_signer.to_account_info(),
            rent: self.rent.to_account_info(),
            account: self.vesting_vault.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
