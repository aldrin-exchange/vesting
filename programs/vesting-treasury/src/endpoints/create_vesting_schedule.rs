use crate::prelude::*;

use anchor_spl::token::{Mint, Token, TokenAccount};

// TODO: not forget to add anchor_spl to changelog
// TODO: if program_authority will be a multisig account, does it
// make sense to be the payer of the init accounts?

#[derive(Accounts)]
pub struct CreateVestingSchedule<'info> {
    #[account(mut)]
    pub program_authority: Signer<'info>,
    #[account(
        constraint = treasury.programdata_address()? == Some(program_authority.key())
            @ err::acc("Signer isn't program's authority"),
    )]
    pub treasury: Program<'info, crate::program::VestingTreasury>,
    #[account(
        init,
        payer = program_authority,
        space = Vesting::space(),
    )]
    pub vesting: Account<'info, Vesting>,
    #[account(
        seeds = [Vesting::SIGNER_PDA_PREFIX, vesting.key().as_ref()],
        bump
    )]
    pub vesting_signer: AccountInfo<'info>,
    #[account(
        init,
        payer = program_authority,
        space = TokenAccount::LEN,
        owner = token_program.key(),
        seeds = [Vesting::VAULT_PREFIX, vesting.key().as_ref()],
        bump,
    )]
    pub vesting_vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(
        constraint = vestee_wallet.mint == mint.key()
        @ err::acc("Vestee wallet must of correct mint")
    )]
    pub vestee_wallet: Account<'info, TokenAccount>,
    // TODO: Are these going to be needed?
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    /// CHECK: UNSAFE_CODES.md#token
    pub rent: AccountInfo<'info>,
}

pub fn handle(ctx: Context<CreateVestingSchedule>) -> Result<()> {
    Ok(())
}
