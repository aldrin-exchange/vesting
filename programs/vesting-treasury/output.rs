#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod endpoints {
    pub mod change_vestee_wallet {
        //! Changes vestee wallet in [`Vesting`] account. The admin should be able
        //! to change the vestee wallet such that in case the vestee wallet becomes
        //! compromised, the admin is able to target a different vestee wallet for a
        //! given vesting account.
        use crate::prelude::*;
        use anchor_spl::token::TokenAccount;
        pub struct ChangeVesteeWallet<'info> {
            pub admin: Signer<'info>,
            #[account(mut)]
            pub vesting: Account<'info, Vesting>,
            pub vestee_wallet_new: Account<'info, TokenAccount>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for ChangeVesteeWallet<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let admin: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("admin"))?;
                let vesting: anchor_lang::accounts::account::Account<Vesting> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vesting"))?;
                let vestee_wallet_new: anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vestee_wallet_new"))?;
                if !vesting.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vesting"));
                }
                Ok(ChangeVesteeWallet {
                    admin,
                    vesting,
                    vestee_wallet_new,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for ChangeVesteeWallet<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.admin.to_account_infos());
                account_infos.extend(self.vesting.to_account_infos());
                account_infos.extend(self.vestee_wallet_new.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for ChangeVesteeWallet<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.admin.to_account_metas(None));
                account_metas.extend(self.vesting.to_account_metas(None));
                account_metas.extend(self.vestee_wallet_new.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for ChangeVesteeWallet<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vesting, program_id)
                    .map_err(|e| e.with_account_name("vesting"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_change_vestee_wallet {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`ChangeVesteeWallet`].
            pub struct ChangeVesteeWallet {
                pub admin: anchor_lang::solana_program::pubkey::Pubkey,
                pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
                pub vestee_wallet_new: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for ChangeVesteeWallet
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.admin, writer)?;
                    borsh::BorshSerialize::serialize(&self.vesting, writer)?;
                    borsh::BorshSerialize::serialize(&self.vestee_wallet_new, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for ChangeVesteeWallet {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.admin, true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vesting,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.vestee_wallet_new,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_change_vestee_wallet {
            use super::*;
            /// Generated CPI struct of the accounts for [`ChangeVesteeWallet`].
            pub struct ChangeVesteeWallet<'info> {
                pub admin: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vestee_wallet_new:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for ChangeVesteeWallet<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.admin),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vesting),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.vestee_wallet_new),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for ChangeVesteeWallet<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.admin));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vestee_wallet_new,
                    ));
                    account_infos
                }
            }
        }
        pub fn handle(ctx: Context<ChangeVesteeWallet>) -> Result<()> {
            let accs = ctx.accounts;
            if accs.vesting.admin != accs.admin.key() {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : TreasuryError :: VestingAdminMismatch . name () , error_code_number : TreasuryError :: VestingAdminMismatch . into () , error_msg : TreasuryError :: VestingAdminMismatch . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/vesting-treasury/src/endpoints/change_vestee_wallet.rs" , line : 21u32 , })) , compared_values : None , })) ;
            }
            if accs.vesting.mint != accs.vestee_wallet_new.mint {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : err :: acc ("The new vestee wallet mint must be of correct mint") . name () , error_code_number : err :: acc ("The new vestee wallet mint must be of correct mint") . into () , error_msg : err :: acc ("The new vestee wallet mint must be of correct mint") . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/vesting-treasury/src/endpoints/change_vestee_wallet.rs" , line : 25u32 , })) , compared_values : None , })) ;
            }
            if accs.vesting.vestee_wallet == accs.vestee_wallet_new.key() {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : err :: acc ("The new vestee wallet is the same as the current vestee wallet") . name () , error_code_number : err :: acc ("The new vestee wallet is the same as the current vestee wallet") . into () , error_msg : err :: acc ("The new vestee wallet is the same as the current vestee wallet") . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/vesting-treasury/src/endpoints/change_vestee_wallet.rs" , line : 31u32 , })) , compared_values : None , })) ;
            }
            accs.vesting.vestee_wallet = accs.vestee_wallet_new.key();
            Ok(())
        }
    }
    pub mod close_vesting_schedule {
        //! If the [`Vesting`] is fully vested and has no tokens that remain to be
        //! withdrawn, then the account is empty and can be closed without losing
        //! funds.
        use crate::prelude::*;
        pub struct CloseVestingSchedule<'info> {
            #[account(mut)]
            pub admin: Signer<'info>,
            # [account (mut , constraint = vesting . admin == admin . key () @ err :: acc ("Admin does not own this vesting account") , close = admin ,)]
            pub vesting: Account<'info, Vesting>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for CloseVestingSchedule<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let admin: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("admin"))?;
                let vesting: anchor_lang::accounts::account::Account<Vesting> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vesting"))?;
                if !admin.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("admin"));
                }
                if !vesting.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vesting"));
                }
                if !(vesting.admin == admin.key()) {
                    return Err(anchor_lang::error::Error::from(err::acc(
                        "Admin does not own this vesting account",
                    ))
                    .with_account_name("vesting"));
                }
                if vesting.key() == admin.key() {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintClose,
                    )
                    .with_account_name("vesting"));
                }
                Ok(CloseVestingSchedule { admin, vesting })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CloseVestingSchedule<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.admin.to_account_infos());
                account_infos.extend(self.vesting.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CloseVestingSchedule<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.admin.to_account_metas(None));
                account_metas.extend(self.vesting.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for CloseVestingSchedule<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.admin, program_id)
                    .map_err(|e| e.with_account_name("admin"))?;
                anchor_lang::AccountsClose::close(&self.vesting, self.admin.to_account_info())
                    .map_err(|e| e.with_account_name("vesting"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_close_vesting_schedule {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`CloseVestingSchedule`].
            pub struct CloseVestingSchedule {
                pub admin: anchor_lang::solana_program::pubkey::Pubkey,
                pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for CloseVestingSchedule
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.admin, writer)?;
                    borsh::BorshSerialize::serialize(&self.vesting, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for CloseVestingSchedule {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.admin, true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vesting,
                        false,
                    ));
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_close_vesting_schedule {
            use super::*;
            /// Generated CPI struct of the accounts for [`CloseVestingSchedule`].
            pub struct CloseVestingSchedule<'info> {
                pub admin: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for CloseVestingSchedule<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.admin),
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vesting),
                        false,
                    ));
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for CloseVestingSchedule<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.admin));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
                    account_infos
                }
            }
        }
        pub fn handle(ctx: Context<CloseVestingSchedule>) -> Result<()> {
            let vesting = &ctx.accounts.vesting;
            if vesting.cumulative_vested_amount < vesting.total_vesting_amount {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : err :: acc ("This vesting account is not fully vested") . name () , error_code_number : err :: acc ("This vesting account is not fully vested") . into () , error_msg : err :: acc ("This vesting account is not fully vested") . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/vesting-treasury/src/endpoints/close_vesting_schedule.rs" , line : 23u32 , })) , compared_values : None , })) ;
            }
            if vesting.cumulative_vested_amount > vesting.cumulative_withdrawn_amount {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : err :: acc ("This vested tokens of this vesting account are not fully withdrawn") . name () , error_code_number : err :: acc ("This vested tokens of this vesting account are not fully withdrawn") . into () , error_msg : err :: acc ("This vested tokens of this vesting account are not fully withdrawn") . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/vesting-treasury/src/endpoints/close_vesting_schedule.rs" , line : 27u32 , })) , compared_values : None , })) ;
            }
            Ok(())
        }
    }
    pub mod create_vesting_schedule {
        //! Initializes new [`Vesting`] account. After this call,
        //! the admin can fund the vesting vault such that the tokens
        //! become available to the beneficiary as they vest over time.
        use crate::prelude::*;
        use anchor_spl::token::{self, Mint, Token, TokenAccount};
        pub struct CreateVestingSchedule<'info> {
            #[account(mut)]
            pub admin: Signer<'info>,
            # [account (init , payer = admin , space = Vesting :: space ())]
            pub vesting: Account<'info, Vesting>,
            /// CHECK: UNSAFE_CODES.md#signer
            # [account (seeds = [Vesting :: SIGNER_PDA_PREFIX , vesting . key () . as_ref ()] , bump)]
            pub vesting_signer: AccountInfo<'info>,
            /// CHECK: UNSAFE_CODES.md#token
            # [account (init , payer = admin , space = TokenAccount :: LEN , owner = token_program . key () , seeds = [Vesting :: VAULT_PREFIX , vesting . key () . as_ref ()] , bump ,)]
            pub vesting_vault: AccountInfo<'info>,
            pub mint: Account<'info, Mint>,
            # [account (constraint = vestee_wallet . mint == mint . key () @ err :: acc ("Vestee wallet must be of correct mint"))]
            pub vestee_wallet: Account<'info, TokenAccount>,
            pub token_program: Program<'info, Token>,
            pub system_program: Program<'info, System>,
            /// CHECK: UNSAFE_CODES.md#token
            pub rent: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for CreateVestingSchedule<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let admin: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("admin"))?;
                if accounts.is_empty() {
                    return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
                }
                let vesting = &accounts[0];
                *accounts = &accounts[1..];
                let vesting_signer: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vesting_signer"))?;
                if accounts.is_empty() {
                    return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
                }
                let vesting_vault = &accounts[0];
                *accounts = &accounts[1..];
                let mint: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("mint"))?;
                let vestee_wallet: anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vestee_wallet"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("system_program"))?;
                let rent: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("rent"))?;
                let __anchor_rent = Rent::get()?;
                let vesting = {
                    let actual_field = vesting.to_account_info();
                    let actual_owner = actual_field.owner;
                    let space = Vesting::space();
                    let pa: anchor_lang::accounts::account::Account<Vesting> = if !false
                        || actual_owner == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = admin.to_account_info();
                        let __current_lamports = vesting.lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: vesting.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context.with_signer(&[]),
                                lamports,
                                space as u64,
                                program_id,
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: vesting.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: vesting.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context.with_signer(&[]),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: vesting.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context.with_signer(&[]),
                                program_id,
                            )?;
                        }
                        anchor_lang::accounts::account::Account::try_from_unchecked(&vesting)?
                    } else {
                        anchor_lang::accounts::account::Account::try_from(&vesting)?
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("vesting")
                            .with_values((space, actual_field.data_len())));
                        }
                        if actual_owner != program_id {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("vesting")
                            .with_pubkeys((*actual_owner, *program_id)));
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("vesting"));
                            }
                        }
                    }
                    pa
                };
                if !vesting.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vesting"));
                }
                if !vesting.to_account_info().is_signer {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSigner,
                    )
                    .with_account_name("vesting"));
                }
                if !__anchor_rent.is_exempt(
                    vesting.to_account_info().lamports(),
                    vesting.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("vesting"));
                }
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[Vesting::VAULT_PREFIX, vesting.key().as_ref()],
                    program_id,
                );
                __bumps.insert("vesting_vault".to_string(), __bump);
                let vesting_vault = {
                    let actual_field = vesting_vault.to_account_info();
                    let actual_owner = actual_field.owner;
                    let space = TokenAccount::LEN;
                    let pa: AccountInfo = if !false
                        || actual_owner == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = admin.to_account_info();
                        let __current_lamports = vesting_vault.lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: vesting_vault.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context.with_signer(&[&[
                                    Vesting::VAULT_PREFIX,
                                    vesting.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                lamports,
                                space as u64,
                                &token_program.key(),
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: vesting_vault.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: vesting_vault.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context.with_signer(&[&[
                                    Vesting::VAULT_PREFIX,
                                    vesting.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: vesting_vault.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context.with_signer(&[&[
                                    Vesting::VAULT_PREFIX,
                                    vesting.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                &token_program.key(),
                            )?;
                        }
                        vesting_vault.to_account_info()
                    } else {
                        vesting_vault.to_account_info()
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("vesting_vault")
                            .with_values((space, actual_field.data_len())));
                        }
                        if actual_owner != &token_program.key() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("vesting_vault")
                            .with_pubkeys((*actual_owner, *&token_program.key())));
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("vesting_vault"));
                            }
                        }
                    }
                    pa
                };
                if vesting_vault.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vesting_vault")
                    .with_pubkeys((vesting_vault.key(), __pda_address)));
                }
                if !vesting_vault.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vesting_vault"));
                }
                {
                    let my_owner = AsRef::<AccountInfo>::as_ref(&vesting_vault).owner;
                    let owner_address = token_program.key();
                    if my_owner != &owner_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintOwner,
                        )
                        .with_account_name("vesting_vault")
                        .with_pubkeys((*my_owner, owner_address)));
                    }
                }
                if !__anchor_rent.is_exempt(
                    vesting_vault.to_account_info().lamports(),
                    vesting_vault.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("vesting_vault"));
                }
                if !admin.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("admin"));
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[Vesting::SIGNER_PDA_PREFIX, vesting.key().as_ref()],
                    &program_id,
                );
                __bumps.insert("vesting_signer".to_string(), __bump);
                if vesting_signer.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vesting_signer")
                    .with_pubkeys((vesting_signer.key(), __pda_address)));
                }
                if !(vestee_wallet.mint == mint.key()) {
                    return Err(anchor_lang::error::Error::from(err::acc(
                        "Vestee wallet must be of correct mint",
                    ))
                    .with_account_name("vestee_wallet"));
                }
                Ok(CreateVestingSchedule {
                    admin,
                    vesting,
                    vesting_signer,
                    vesting_vault,
                    mint,
                    vestee_wallet,
                    token_program,
                    system_program,
                    rent,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CreateVestingSchedule<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.admin.to_account_infos());
                account_infos.extend(self.vesting.to_account_infos());
                account_infos.extend(self.vesting_signer.to_account_infos());
                account_infos.extend(self.vesting_vault.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.vestee_wallet.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CreateVestingSchedule<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.admin.to_account_metas(None));
                account_metas.extend(self.vesting.to_account_metas(Some(true)));
                account_metas.extend(self.vesting_signer.to_account_metas(None));
                account_metas.extend(self.vesting_vault.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.vestee_wallet.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for CreateVestingSchedule<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.admin, program_id)
                    .map_err(|e| e.with_account_name("admin"))?;
                anchor_lang::AccountsExit::exit(&self.vesting, program_id)
                    .map_err(|e| e.with_account_name("vesting"))?;
                anchor_lang::AccountsExit::exit(&self.vesting_vault, program_id)
                    .map_err(|e| e.with_account_name("vesting_vault"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_create_vesting_schedule {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`CreateVestingSchedule`].
            pub struct CreateVestingSchedule {
                pub admin: anchor_lang::solana_program::pubkey::Pubkey,
                pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: UNSAFE_CODES.md#signer
                pub vesting_signer: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: UNSAFE_CODES.md#token
                pub vesting_vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub vestee_wallet: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: UNSAFE_CODES.md#token
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for CreateVestingSchedule
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.admin, writer)?;
                    borsh::BorshSerialize::serialize(&self.vesting, writer)?;
                    borsh::BorshSerialize::serialize(&self.vesting_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.vesting_vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.vestee_wallet, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for CreateVestingSchedule {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.admin, true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vesting,
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.vesting_signer,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vesting_vault,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.mint, false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.vestee_wallet,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent, false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_create_vesting_schedule {
            use super::*;
            /// Generated CPI struct of the accounts for [`CreateVestingSchedule`].
            pub struct CreateVestingSchedule<'info> {
                pub admin: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: UNSAFE_CODES.md#signer
                pub vesting_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: UNSAFE_CODES.md#token
                pub vesting_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vestee_wallet: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: UNSAFE_CODES.md#token
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for CreateVestingSchedule<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.admin),
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vesting),
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.vesting_signer),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vesting_vault),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.mint),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.vestee_wallet),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for CreateVestingSchedule<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.admin));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vesting_signer,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vesting_vault,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.mint));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vestee_wallet,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                    account_infos
                }
            }
        }
        pub fn handle(
            ctx: Context<CreateVestingSchedule>,
            vesting_amount: TokenAmount,
            start_ts: TimeStamp,
            cliff_periods: u64,
            total_periods: u64,
            period_type: u32,
        ) -> Result<()> {
            if period_type > 2 {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : err :: arg ("The current contract version only supports \
             vesting schedules with daily or monthly periods") . name () , error_code_number : err :: arg ("The current contract version only supports \
             vesting schedules with daily or monthly periods") . into () , error_msg : err :: arg ("The current contract version only supports \
             vesting schedules with daily or monthly periods") . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/vesting-treasury/src/endpoints/create_vesting_schedule.rs" , line : 55u32 , })) , compared_values : None , })) ;
            }
            if cliff_periods > total_periods {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : err :: arg ("The number of cliff periods cannot be higher than total number of periods") . name () , error_code_number : err :: arg ("The number of cliff periods cannot be higher than total number of periods") . into () , error_msg : err :: arg ("The number of cliff periods cannot be higher than total number of periods") . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/vesting-treasury/src/endpoints/create_vesting_schedule.rs" , line : 62u32 , })) , compared_values : None , })) ;
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
            ::solana_program::log::sol_log("Initializing vesting vault");
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
    }
    pub mod fund_vesting_vault {
        //! Funds [`vesting_vault`] account. In order for the vested tokens to be
        //! withdrawn from the vesting vault, the vault first needs to be funded with
        //! tokens of the vesting mint. We track the amount of tokens that the admin
        //! needs to deposit in vesting vault in order to fulfill the promises stated
        //! on the vesting schedule via the [`Vesting`] field [`unfunded_liability`].
        use crate::prelude::*;
        use anchor_spl::token::{self, Token, TokenAccount};
        pub struct FundVestingVault<'info> {
            #[account(mut)]
            pub wallet_authority: Signer<'info>,
            #[account(mut)]
            pub vesting: Account<'info, Vesting>,
            # [account (mut , constraint = vesting_vault . key () == vesting . vault . key () @ err :: acc ("Vault input does not match the vault in the vesting account"))]
            pub vesting_vault: Account<'info, TokenAccount>,
            # [account (mut , constraint = funding_wallet . mint == vesting . mint . key () @ err :: acc ("Funding wallet must be of correct mint"))]
            pub funding_wallet: Account<'info, TokenAccount>,
            pub token_program: Program<'info, Token>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for FundVestingVault<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let wallet_authority: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("wallet_authority"))?;
                let vesting: anchor_lang::accounts::account::Account<Vesting> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vesting"))?;
                let vesting_vault: anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vesting_vault"))?;
                let funding_wallet: anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("funding_wallet"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                if !wallet_authority.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("wallet_authority"));
                }
                if !vesting.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vesting"));
                }
                if !vesting_vault.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vesting_vault"));
                }
                if !(vesting_vault.key() == vesting.vault.key()) {
                    return Err(anchor_lang::error::Error::from(err::acc(
                        "Vault input does not match the vault in the vesting account",
                    ))
                    .with_account_name("vesting_vault"));
                }
                if !funding_wallet.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("funding_wallet"));
                }
                if !(funding_wallet.mint == vesting.mint.key()) {
                    return Err(anchor_lang::error::Error::from(err::acc(
                        "Funding wallet must be of correct mint",
                    ))
                    .with_account_name("funding_wallet"));
                }
                Ok(FundVestingVault {
                    wallet_authority,
                    vesting,
                    vesting_vault,
                    funding_wallet,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for FundVestingVault<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.wallet_authority.to_account_infos());
                account_infos.extend(self.vesting.to_account_infos());
                account_infos.extend(self.vesting_vault.to_account_infos());
                account_infos.extend(self.funding_wallet.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for FundVestingVault<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.wallet_authority.to_account_metas(None));
                account_metas.extend(self.vesting.to_account_metas(None));
                account_metas.extend(self.vesting_vault.to_account_metas(None));
                account_metas.extend(self.funding_wallet.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for FundVestingVault<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.wallet_authority, program_id)
                    .map_err(|e| e.with_account_name("wallet_authority"))?;
                anchor_lang::AccountsExit::exit(&self.vesting, program_id)
                    .map_err(|e| e.with_account_name("vesting"))?;
                anchor_lang::AccountsExit::exit(&self.vesting_vault, program_id)
                    .map_err(|e| e.with_account_name("vesting_vault"))?;
                anchor_lang::AccountsExit::exit(&self.funding_wallet, program_id)
                    .map_err(|e| e.with_account_name("funding_wallet"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_fund_vesting_vault {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`FundVestingVault`].
            pub struct FundVestingVault {
                pub wallet_authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
                pub vesting_vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub funding_wallet: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for FundVestingVault
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.wallet_authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.vesting, writer)?;
                    borsh::BorshSerialize::serialize(&self.vesting_vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.funding_wallet, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for FundVestingVault {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.wallet_authority,
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vesting,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vesting_vault,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.funding_wallet,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_fund_vesting_vault {
            use super::*;
            /// Generated CPI struct of the accounts for [`FundVestingVault`].
            pub struct FundVestingVault<'info> {
                pub wallet_authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vesting_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub funding_wallet: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for FundVestingVault<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.wallet_authority),
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vesting),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vesting_vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.funding_wallet),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for FundVestingVault<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.wallet_authority,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vesting_vault,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.funding_wallet,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        pub fn handle(ctx: Context<FundVestingVault>, funding_amount: TokenAmount) -> Result<()> {
            let accs = ctx.accounts;
            token::transfer(
                accs.as_transfer_funds_from_funding_wallet_to_vault_context(),
                funding_amount.amount,
            )?;
            accs.vesting.vault_balance =
                TokenAmount::new(accs.vesting.vault_balance.amount + funding_amount.amount);
            accs.vesting.update_unfunded_liability()?;
            Ok(())
        }
        impl<'info> FundVestingVault<'info> {
            fn as_transfer_funds_from_funding_wallet_to_vault_context(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
                let cpi_accounts = token::Transfer {
                    from: self.funding_wallet.to_account_info(),
                    to: self.vesting_vault.to_account_info(),
                    authority: self.wallet_authority.to_account_info(),
                };
                let cpi_program = self.token_program.to_account_info();
                CpiContext::new(cpi_program, cpi_accounts)
            }
        }
    }
    pub mod update_vested_tokens {
        //! Calculates and updates the amount of tokens vested in the vesting schedule.
        //! The endpoint uses the solana clock account to access the runtime clock and
        //! compare it against the vesting dates to calculate how many periods have
        //! vested. Based on the amount of vested periods the endpoint then computes
        //! the pro-rata amount of tokens that are vested.
        use crate::prelude::*;
        use std::collections::HashSet;
        pub struct UpdateVestedTokens<'info> {
            #[account(mut)]
            pub vesting: Account<'info, Vesting>,
            pub clock: Sysvar<'info, Clock>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for UpdateVestedTokens<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let vesting: anchor_lang::accounts::account::Account<Vesting> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vesting"))?;
                let clock: Sysvar<Clock> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("clock"))?;
                if !vesting.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vesting"));
                }
                Ok(UpdateVestedTokens { vesting, clock })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateVestedTokens<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.vesting.to_account_infos());
                account_infos.extend(self.clock.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for UpdateVestedTokens<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.vesting.to_account_metas(None));
                account_metas.extend(self.clock.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for UpdateVestedTokens<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vesting, program_id)
                    .map_err(|e| e.with_account_name("vesting"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_update_vested_tokens {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`UpdateVestedTokens`].
            pub struct UpdateVestedTokens {
                pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
                pub clock: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for UpdateVestedTokens
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.vesting, writer)?;
                    borsh::BorshSerialize::serialize(&self.clock, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for UpdateVestedTokens {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vesting,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock, false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_update_vested_tokens {
            use super::*;
            /// Generated CPI struct of the accounts for [`UpdateVestedTokens`].
            pub struct UpdateVestedTokens<'info> {
                pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for UpdateVestedTokens<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vesting),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateVestedTokens<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.clock));
                    account_infos
                }
            }
        }
        pub fn handle(ctx: Context<UpdateVestedTokens>) -> Result<()> {
            let accs = ctx.accounts;
            let clock_ts = accs.clock.unix_timestamp;
            let supported_types: HashSet<PeriodType> =
                HashSet::from([PeriodType::Monthly, PeriodType::Daily]);
            if !supported_types.contains(&accs.vesting.period_type) {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : err :: arg ("The current contract version only supports \
            vesting schedules with daily or monthly periods") . name () , error_code_number : err :: arg ("The current contract version only supports \
            vesting schedules with daily or monthly periods") . into () , error_msg : err :: arg ("The current contract version only supports \
            vesting schedules with daily or monthly periods") . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/vesting-treasury/src/endpoints/update_vested_tokens.rs" , line : 26u32 , })) , compared_values : None , })) ;
            }
            accs.vesting.update_vested_tokens(clock_ts)?;
            accs.vesting.update_unfunded_liability()?;
            Ok(())
        }
    }
    pub mod withdraw_vested_tokens {
        //! Endpoint for users to withdraw vested tokens from [`vesting_vault`] to
        //! the [`vestee_wallet`]. Whilst the endpoint is permissionless, only tokens
        //! that have been vested and subsequently funded by the administrator or
        //! any other agent, will be avaialble for transfer. The endpoint is made
        //! permissionless to more easily allow for automation.
        use crate::prelude::*;
        use anchor_spl::token::{self, Token, TokenAccount};
        pub struct WithdrawVestedTokens<'info> {
            #[account(mut)]
            pub vesting: Account<'info, Vesting>,
            # [account (mut , constraint = vesting_vault . key () == vesting . vault . key () @ err :: acc ("Vault input does not match the vault in the vesting account"))]
            pub vesting_vault: Account<'info, TokenAccount>,
            /// CHECK: UNSAFE_CODES.md#signer
            # [account (seeds = [Vesting :: SIGNER_PDA_PREFIX , vesting . key () . as_ref ()] , bump)]
            pub vesting_signer: AccountInfo<'info>,
            # [account (mut , constraint = vestee_wallet . key () == vesting . vestee_wallet . key () @ err :: acc ("Vestee wallet input does not match the \
         vestee wallet in the vesting account"))]
            pub vestee_wallet: Account<'info, TokenAccount>,
            pub token_program: Program<'info, Token>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for WithdrawVestedTokens<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let vesting: anchor_lang::accounts::account::Account<Vesting> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vesting"))?;
                let vesting_vault: anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vesting_vault"))?;
                let vesting_signer: AccountInfo =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vesting_signer"))?;
                let vestee_wallet: anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("vestee_wallet"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("token_program"))?;
                if !vesting.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vesting"));
                }
                if !vesting_vault.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vesting_vault"));
                }
                if !(vesting_vault.key() == vesting.vault.key()) {
                    return Err(anchor_lang::error::Error::from(err::acc(
                        "Vault input does not match the vault in the vesting account",
                    ))
                    .with_account_name("vesting_vault"));
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[Vesting::SIGNER_PDA_PREFIX, vesting.key().as_ref()],
                    &program_id,
                );
                __bumps.insert("vesting_signer".to_string(), __bump);
                if vesting_signer.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vesting_signer")
                    .with_pubkeys((vesting_signer.key(), __pda_address)));
                }
                if !vestee_wallet.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vestee_wallet"));
                }
                if !(vestee_wallet.key() == vesting.vestee_wallet.key()) {
                    return Err(anchor_lang::error::Error::from(err::acc(
                        "Vestee wallet input does not match the \
         vestee wallet in the vesting account",
                    ))
                    .with_account_name("vestee_wallet"));
                }
                Ok(WithdrawVestedTokens {
                    vesting,
                    vesting_vault,
                    vesting_signer,
                    vestee_wallet,
                    token_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawVestedTokens<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.vesting.to_account_infos());
                account_infos.extend(self.vesting_vault.to_account_infos());
                account_infos.extend(self.vesting_signer.to_account_infos());
                account_infos.extend(self.vestee_wallet.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for WithdrawVestedTokens<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.vesting.to_account_metas(None));
                account_metas.extend(self.vesting_vault.to_account_metas(None));
                account_metas.extend(self.vesting_signer.to_account_metas(None));
                account_metas.extend(self.vestee_wallet.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for WithdrawVestedTokens<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vesting, program_id)
                    .map_err(|e| e.with_account_name("vesting"))?;
                anchor_lang::AccountsExit::exit(&self.vesting_vault, program_id)
                    .map_err(|e| e.with_account_name("vesting_vault"))?;
                anchor_lang::AccountsExit::exit(&self.vestee_wallet, program_id)
                    .map_err(|e| e.with_account_name("vestee_wallet"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_withdraw_vested_tokens {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`WithdrawVestedTokens`].
            pub struct WithdrawVestedTokens {
                pub vesting: anchor_lang::solana_program::pubkey::Pubkey,
                pub vesting_vault: anchor_lang::solana_program::pubkey::Pubkey,
                /// CHECK: UNSAFE_CODES.md#signer
                pub vesting_signer: anchor_lang::solana_program::pubkey::Pubkey,
                pub vestee_wallet: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for WithdrawVestedTokens
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.vesting, writer)?;
                    borsh::BorshSerialize::serialize(&self.vesting_vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.vesting_signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.vestee_wallet, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for WithdrawVestedTokens {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vesting,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vesting_vault,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.vesting_signer,
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vestee_wallet,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_withdraw_vested_tokens {
            use super::*;
            /// Generated CPI struct of the accounts for [`WithdrawVestedTokens`].
            pub struct WithdrawVestedTokens<'info> {
                pub vesting: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vesting_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                /// CHECK: UNSAFE_CODES.md#signer
                pub vesting_signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vestee_wallet: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for WithdrawVestedTokens<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vesting),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vesting_vault),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.vesting_signer),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vestee_wallet),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawVestedTokens<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vesting));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vesting_vault,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vesting_signer,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.vestee_wallet,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos
                }
            }
        }
        pub fn handle(
            ctx: Context<WithdrawVestedTokens>,
            withdraw_amount: TokenAmount,
        ) -> Result<()> {
            let accs = ctx.accounts;
            let signer_bump_seed = *ctx.bumps.get("vesting_signer").unwrap();
            let liability = accs.vesting.get_current_liability()?;
            if withdraw_amount.amount > liability {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : err :: arg ("The amount of tokens to withdraw is bigger than\
            the amount of vested tokens to be withdrawn") . name () , error_code_number : err :: arg ("The amount of tokens to withdraw is bigger than\
            the amount of vested tokens to be withdrawn") . into () , error_msg : err :: arg ("The amount of tokens to withdraw is bigger than\
            the amount of vested tokens to be withdrawn") . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/vesting-treasury/src/endpoints/withdraw_vested_tokens.rs" , line : 44u32 , })) , compared_values : None , })) ;
            }
            if withdraw_amount.amount > accs.vesting.vault_balance.amount {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : err :: arg ("The amount of tokens to withdraw is higher \
            than the amount of tokens currently in vault, \
            it seems the vault is partially unfunded") . name () , error_code_number : err :: arg ("The amount of tokens to withdraw is higher \
            than the amount of tokens currently in vault, \
            it seems the vault is partially unfunded") . into () , error_msg : err :: arg ("The amount of tokens to withdraw is higher \
            than the amount of tokens currently in vault, \
            it seems the vault is partially unfunded") . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/vesting-treasury/src/endpoints/withdraw_vested_tokens.rs" , line : 51u32 , })) , compared_values : None , })) ;
            }
            let signer_seeds = &[
                Vesting::SIGNER_PDA_PREFIX,
                &accs.vesting.key().to_bytes()[..],
                &[signer_bump_seed],
            ];
            token::transfer(
                accs.as_transfer_funds_from_vesting_vault_to_vestee_wallet_context()
                    .with_signer(&[&signer_seeds[..]]),
                withdraw_amount.amount,
            )?;
            accs.vesting.vault_balance =
                TokenAmount::new(accs.vesting.vault_balance.amount - withdraw_amount.amount);
            accs.vesting.cumulative_withdrawn_amount = TokenAmount::new(
                accs.vesting.cumulative_withdrawn_amount.amount + withdraw_amount.amount,
            );
            Ok(())
        }
        impl<'info> WithdrawVestedTokens<'info> {
            fn as_transfer_funds_from_vesting_vault_to_vestee_wallet_context(
                &self,
            ) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
                let cpi_accounts = token::Transfer {
                    from: self.vesting_vault.to_account_info(),
                    to: self.vestee_wallet.to_account_info(),
                    authority: self.vesting_signer.to_account_info(),
                };
                let cpi_program = self.token_program.to_account_info();
                CpiContext::new(cpi_program, cpi_accounts)
            }
        }
    }
    pub use change_vestee_wallet::*;
    pub use close_vesting_schedule::*;
    pub use create_vesting_schedule::*;
    pub use fund_vesting_vault::*;
    pub use update_vested_tokens::*;
    pub use withdraw_vested_tokens::*;
}
pub mod err {
    use crate::prelude::*;
    use std::fmt::Display;
    #[repr(u32)]
    pub enum TreasuryError {
        /// Use this error via the [`acc`] function to provide more background
        /// about the issue.
        InvalidAccountInput,
        /// Use this error via the [`arg`] function to provide more background
        /// about the issue.
        InvalidArg,
        VestingAdminMismatch,
        /// Use this error for program paths which should never be reached if the
        /// program logic works as intended.
        InvariantViolation,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TreasuryError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                TreasuryError::InvalidAccountInput => {
                    ::core::fmt::Formatter::write_str(f, "InvalidAccountInput")
                }
                TreasuryError::InvalidArg => ::core::fmt::Formatter::write_str(f, "InvalidArg"),
                TreasuryError::VestingAdminMismatch => {
                    ::core::fmt::Formatter::write_str(f, "VestingAdminMismatch")
                }
                TreasuryError::InvariantViolation => {
                    ::core::fmt::Formatter::write_str(f, "InvariantViolation")
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TreasuryError {
        #[inline]
        fn clone(&self) -> TreasuryError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TreasuryError {}
    impl TreasuryError {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                TreasuryError::InvalidAccountInput => "InvalidAccountInput".to_string(),
                TreasuryError::InvalidArg => "InvalidArg".to_string(),
                TreasuryError::VestingAdminMismatch => "VestingAdminMismatch".to_string(),
                TreasuryError::InvariantViolation => "InvariantViolation".to_string(),
            }
        }
    }
    impl From<TreasuryError> for u32 {
        fn from(e: TreasuryError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<TreasuryError> for anchor_lang::error::Error {
        fn from(error_code: TreasuryError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for TreasuryError {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                TreasuryError::InvalidAccountInput => {
                    let result = fmt.write_fmt(::core::fmt::Arguments::new_v1(
                        &["Provided account breaks some constraints, see logs for more info"],
                        &[],
                    ));
                    result
                }
                TreasuryError::InvalidArg => {
                    let result = fmt.write_fmt(::core::fmt::Arguments::new_v1(
                        &["One of the provided input arguments is invalid"],
                        &[],
                    ));
                    result
                }
                TreasuryError::VestingAdminMismatch => {
                    let result = fmt.write_fmt(::core::fmt::Arguments::new_v1(
                        &["Vesting admin does not match the provided signer"],
                        &[],
                    ));
                    result
                }
                TreasuryError::InvariantViolation => {
                    let result = fmt.write_fmt(::core::fmt::Arguments::new_v1(
                        &["There\'s a bug in the program, see logs for more info"],
                        &[],
                    ));
                    result
                }
            }
        }
    }
    pub fn acc(msg: impl Display) -> TreasuryError {
        ::solana_program::log::sol_log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["[InvalidAccountInput] "],
                &[::core::fmt::ArgumentV1::new_display(&msg)],
            ));
            res
        });
        TreasuryError::InvalidAccountInput
    }
    pub fn arg(msg: impl Display) -> TreasuryError {
        ::solana_program::log::sol_log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["[InvalidArg] "],
                &[::core::fmt::ArgumentV1::new_display(&msg)],
            ));
            res
        });
        TreasuryError::InvalidArg
    }
}
pub mod models {
    pub mod vesting {
        use crate::prelude::*;
        use chrono::Duration;
        use std::mem;
        pub struct Vesting {
            /// The authority of this Vesting account.
            pub admin: Pubkey,
            /// The vestee wallet of this Vesting account, that should receive the
            /// vested funds.
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
            ///The unfunded liability is the amount of vested tokens that a user
            /// is already allowed to withdraw but are still not available in the
            /// vesting vault, therefore constituting a liability on behalf of
            /// the funder.
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
        impl borsh::ser::BorshSerialize for Vesting
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            TokenAmount: borsh::ser::BorshSerialize,
            TokenAmount: borsh::ser::BorshSerialize,
            TokenAmount: borsh::ser::BorshSerialize,
            TokenAmount: borsh::ser::BorshSerialize,
            TokenAmount: borsh::ser::BorshSerialize,
            TimeStamp: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            PeriodType: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.admin, writer)?;
                borsh::BorshSerialize::serialize(&self.vestee_wallet, writer)?;
                borsh::BorshSerialize::serialize(&self.mint, writer)?;
                borsh::BorshSerialize::serialize(&self.vault, writer)?;
                borsh::BorshSerialize::serialize(&self.total_vesting_amount, writer)?;
                borsh::BorshSerialize::serialize(&self.cumulative_vested_amount, writer)?;
                borsh::BorshSerialize::serialize(&self.cumulative_withdrawn_amount, writer)?;
                borsh::BorshSerialize::serialize(&self.vault_balance, writer)?;
                borsh::BorshSerialize::serialize(&self.unfunded_liability, writer)?;
                borsh::BorshSerialize::serialize(&self.start_ts, writer)?;
                borsh::BorshSerialize::serialize(&self.total_periods, writer)?;
                borsh::BorshSerialize::serialize(&self.cliff_periods, writer)?;
                borsh::BorshSerialize::serialize(&self.period_type, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Vesting
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            TokenAmount: borsh::BorshDeserialize,
            TokenAmount: borsh::BorshDeserialize,
            TokenAmount: borsh::BorshDeserialize,
            TokenAmount: borsh::BorshDeserialize,
            TokenAmount: borsh::BorshDeserialize,
            TimeStamp: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            PeriodType: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    admin: borsh::BorshDeserialize::deserialize(buf)?,
                    vestee_wallet: borsh::BorshDeserialize::deserialize(buf)?,
                    mint: borsh::BorshDeserialize::deserialize(buf)?,
                    vault: borsh::BorshDeserialize::deserialize(buf)?,
                    total_vesting_amount: borsh::BorshDeserialize::deserialize(buf)?,
                    cumulative_vested_amount: borsh::BorshDeserialize::deserialize(buf)?,
                    cumulative_withdrawn_amount: borsh::BorshDeserialize::deserialize(buf)?,
                    vault_balance: borsh::BorshDeserialize::deserialize(buf)?,
                    unfunded_liability: borsh::BorshDeserialize::deserialize(buf)?,
                    start_ts: borsh::BorshDeserialize::deserialize(buf)?,
                    total_periods: borsh::BorshDeserialize::deserialize(buf)?,
                    cliff_periods: borsh::BorshDeserialize::deserialize(buf)?,
                    period_type: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Vesting {
            #[inline]
            fn clone(&self) -> Vesting {
                Vesting {
                    admin: ::core::clone::Clone::clone(&self.admin),
                    vestee_wallet: ::core::clone::Clone::clone(&self.vestee_wallet),
                    mint: ::core::clone::Clone::clone(&self.mint),
                    vault: ::core::clone::Clone::clone(&self.vault),
                    total_vesting_amount: ::core::clone::Clone::clone(&self.total_vesting_amount),
                    cumulative_vested_amount: ::core::clone::Clone::clone(
                        &self.cumulative_vested_amount,
                    ),
                    cumulative_withdrawn_amount: ::core::clone::Clone::clone(
                        &self.cumulative_withdrawn_amount,
                    ),
                    vault_balance: ::core::clone::Clone::clone(&self.vault_balance),
                    unfunded_liability: ::core::clone::Clone::clone(&self.unfunded_liability),
                    start_ts: ::core::clone::Clone::clone(&self.start_ts),
                    total_periods: ::core::clone::Clone::clone(&self.total_periods),
                    cliff_periods: ::core::clone::Clone::clone(&self.cliff_periods),
                    period_type: ::core::clone::Clone::clone(&self.period_type),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for Vesting {
            fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
                if writer
                    .write_all(&[100, 149, 66, 138, 95, 200, 128, 241])
                    .is_err()
                {
                    return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for Vesting {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [100, 149, 66, 138, 95, 200, 128, 241].len() {
                    return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
                }
                let given_disc = &buf[..8];
                if &[100, 149, 66, 138, 95, 200, 128, 241] != given_disc {
                    return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into());
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for Vesting {
            fn discriminator() -> [u8; 8] {
                [100, 149, 66, 138, 95, 200, 128, 241]
            }
        }
        #[automatically_derived]
        impl anchor_lang::Owner for Vesting {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Vesting {
            #[inline]
            fn default() -> Vesting {
                Vesting {
                    admin: ::core::default::Default::default(),
                    vestee_wallet: ::core::default::Default::default(),
                    mint: ::core::default::Default::default(),
                    vault: ::core::default::Default::default(),
                    total_vesting_amount: ::core::default::Default::default(),
                    cumulative_vested_amount: ::core::default::Default::default(),
                    cumulative_withdrawn_amount: ::core::default::Default::default(),
                    vault_balance: ::core::default::Default::default(),
                    unfunded_liability: ::core::default::Default::default(),
                    start_ts: ::core::default::Default::default(),
                    total_periods: ::core::default::Default::default(),
                    cliff_periods: ::core::default::Default::default(),
                    period_type: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Vesting {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "admin",
                    "vestee_wallet",
                    "mint",
                    "vault",
                    "total_vesting_amount",
                    "cumulative_vested_amount",
                    "cumulative_withdrawn_amount",
                    "vault_balance",
                    "unfunded_liability",
                    "start_ts",
                    "total_periods",
                    "cliff_periods",
                    "period_type",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.admin,
                    &&self.vestee_wallet,
                    &&self.mint,
                    &&self.vault,
                    &&self.total_vesting_amount,
                    &&self.cumulative_vested_amount,
                    &&self.cumulative_withdrawn_amount,
                    &&self.vault_balance,
                    &&self.unfunded_liability,
                    &&self.start_ts,
                    &&self.total_periods,
                    &&self.cliff_periods,
                    &&self.period_type,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Vesting", names, values)
            }
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
            /// Updates the field `cumulative_vested_amount` in `Vesting`struct based
            /// on the amount of days that have passed. The method receives the
            /// argument `clock_ts`, which stands for clock timestamp. In the endpoint
            /// `updated_vested_tokens` we call this method with `clock_ts` being the
            /// the current timestamp given by the runtime.
            ///
            /// Vesting schedules have a cliff period following by a period where the
            /// schedule vests periodically, usually monthly or daily. The periodicity
            /// is given by the `self.period_type`. As of this contract version only the
            /// type `Monthly` or `Daily` are supported by the endpoint that calls this
            /// method.
            ///
            /// If we find ourselves before the end of the cliff period, the amount of
            /// tokens vested is nill, therefore we perform an early return and do not
            /// update state. If we find ourselves after the end of the full vesting
            /// period then all the tokens will be vested and the state updated
            /// accordingly.
            pub fn update_vested_tokens(&mut self, clock_ts: i64) -> Result<()> {
                let current_dt: DateTime<Utc> =
                    DateTime::from_utc(NaiveDateTime::from_timestamp(clock_ts, 0), Utc);
                let start_dt: DateTime<Utc> =
                    DateTime::from_utc(NaiveDateTime::from_timestamp(self.start_ts.time, 0), Utc);
                let cliff_dt = self.shift_periods(start_dt, self.cliff_periods)?;
                let end_dt = self.shift_periods(start_dt, self.total_periods)?;
                if current_dt < cliff_dt {
                    ::solana_program::log::sol_log(
                        "We are still in the cliff period and \
                therefore there are no vested tokens yet",
                    );
                    return Ok(());
                }
                if current_dt >= end_dt {
                    ::solana_program::log::sol_log("All tokens are fully vested");
                    self.cumulative_vested_amount = self.total_vesting_amount;
                    return Ok(());
                }
                let delta_periods = self.compute_delta_periods(current_dt, cliff_dt)?;
                let cumulative_vested = Decimal::from(self.cliff_periods)
                    .try_add(Decimal::from(delta_periods))?
                    .try_div(Decimal::from(self.total_periods))?
                    .try_mul(Decimal::from(self.total_vesting_amount))?
                    .try_floor()?;
                self.cumulative_vested_amount = TokenAmount::new(cumulative_vested);
                Ok(())
            }
            /// This method computes the amount of periods between two dates. The
            /// operations used to compute the result depend on the type of period
            /// defined by the enum PeriodType.
            ///
            /// The current contract supports the PeriodType of `Daily` and `Monthly`.
            ///
            /// If the type is daily then the calculation is simply the difference in
            /// full days between the cliff date and the current date. Note that the
            /// current date is guaranteed to be higher or equal to the cliff date since
            /// we only call this method after confirm that such condition holds.
            ///
            /// If the type is monthly then depending if both dates are in the same year
            /// or if they are years apart from each other the method will break down
            /// the calcualtion in three steps. The first year, the years in between
            /// and the last year, and will call functions for each step
            pub fn compute_delta_periods(
                &mut self,
                current_dt: DateTime<Utc>,
                cliff_dt: DateTime<Utc>,
            ) -> Result<u64> {
                if current_dt < cliff_dt {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: err::arg(
                                "This function should never be called when current_dt < cliff_dt",
                            )
                            .name(),
                            error_code_number: err::arg(
                                "This function should never be called when current_dt < cliff_dt",
                            )
                            .into(),
                            error_msg: err::arg(
                                "This function should never be called when current_dt < cliff_dt",
                            )
                            .to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/vesting-treasury/src/models/vesting.rs",
                                    line: 162u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                }
                match self.period_type {
                    PeriodType::Daily => {
                        let delta_periods = current_dt
                            .date()
                            .signed_duration_since(cliff_dt.date())
                            .num_days();
                        Ok(delta_periods as u64)
                    }
                    PeriodType::Monthly => {
                        let delta_years = (current_dt.year() - cliff_dt.year()) as u32;
                        let delta_periods = match delta_years {
                            0 => compute_periods_from_cliff_to_current_dt(cliff_dt, current_dt),
                            1 => {
                                compute_periods_from_cliff_to_eoy(cliff_dt)
                                    + compute_periods_from_boy_to_current_dt(cliff_dt, current_dt)
                            }
                            _ => {
                                compute_periods_from_cliff_to_eoy(cliff_dt)
                                    + compute_periods_in_full_years(delta_years)
                                    + compute_periods_from_boy_to_current_dt(cliff_dt, current_dt)
                            }
                        };
                        Ok(delta_periods as u64)
                    }
                    _ => Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: err::acc(
                                "Current program only supports Daily or Monthly PeriodType",
                            )
                            .name(),
                            error_code_number: err::acc(
                                "Current program only supports Daily or Monthly PeriodType",
                            )
                            .into(),
                            error_msg: err::acc(
                                "Current program only supports Daily or Monthly PeriodType",
                            )
                            .to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/vesting-treasury/src/models/vesting.rs",
                                    line: 211u32,
                                },
                            )),
                            compared_values: None,
                        },
                    )),
                }
            }
            /// Shifts a date according to the period defined. If the period defined in
            /// the vesting account is `Monthly` then it will shift the date by n months
            /// and if the period is `Daily` it will shift by n days, where n is the
            /// argument `periods`
            pub fn shift_periods(
                &mut self,
                date: DateTime<Utc>,
                periods: u64,
            ) -> Result<DateTime<Utc>> {
                match self.period_type {
                    PeriodType::Daily => date
                        .checked_add_signed(Duration::days(periods as i64))
                        .ok_or_else(|| {
                            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: TreasuryError::InvariantViolation.name(),
                                error_code_number: TreasuryError::InvariantViolation.into(),
                                error_msg: TreasuryError::InvariantViolation.to_string(),
                                error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                    anchor_lang::error::Source {
                                        filename: "programs/vesting-treasury/src/models/vesting.rs",
                                        line: 225u32,
                                    },
                                )),
                                compared_values: None,
                            })
                        }),
                    PeriodType::Monthly => Ok(shift_months(date, periods as i32)),
                    _ => Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: err::acc(
                                "Current program only supports Daily or Monthly PeriodType",
                            )
                            .name(),
                            error_code_number: err::acc(
                                "Current program only supports Daily or Monthly PeriodType",
                            )
                            .into(),
                            error_msg: err::acc(
                                "Current program only supports Daily or Monthly PeriodType",
                            )
                            .to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/vesting-treasury/src/models/vesting.rs",
                                    line: 227u32,
                                },
                            )),
                            compared_values: None,
                        },
                    )),
                }
            }
            /// It updates unfunded liability of the vesting account. The unfunded
            /// is the amount of vested tokens that a user is already allowed to
            /// withdraw but are still not available in the vesting vault, therefore
            /// constituting a liability on behalf of the funder.
            ///
            /// To calculate the unfunded liabilities we first compute the liability,
            /// which is simply the difference between what has vested and what has
            /// been withdrawn. From that liability we then compare it with the vesting
            /// vault balance to determine if there is any unfunded amount.
            pub fn update_unfunded_liability(&mut self) -> Result<()> {
                let liability = Decimal::from(self.cumulative_vested_amount)
                    .try_sub(Decimal::from(self.cumulative_withdrawn_amount))?
                    .try_round()?;
                if self.vault_balance.amount >= liability {
                    return Ok(());
                }
                let unfunded_liability = liability - self.vault_balance.amount;
                self.unfunded_liability = TokenAmount::new(unfunded_liability);
                Ok(())
            }
            pub fn get_current_liability(&mut self) -> Result<u64> {
                Decimal::from(self.cumulative_vested_amount)
                    .try_sub(Decimal::from(self.cumulative_withdrawn_amount))?
                    .try_round()
            }
        }
        pub enum PeriodType {
            Daily,
            Monthly,
            Quarterly,
            SemiAnnually,
            Yearly,
        }
        impl borsh::de::BorshDeserialize for PeriodType {
            fn deserialize(
                buf: &mut &[u8],
            ) -> core::result::Result<Self, borsh::maybestd::io::Error> {
                let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
                let return_value = match variant_idx {
                    0u8 => PeriodType::Daily,
                    1u8 => PeriodType::Monthly,
                    2u8 => PeriodType::Quarterly,
                    3u8 => PeriodType::SemiAnnually,
                    4u8 => PeriodType::Yearly,
                    _ => {
                        let msg = {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Unexpected variant index: "],
                                &[::core::fmt::ArgumentV1::new_debug(&variant_idx)],
                            ));
                            res
                        };
                        return Err(borsh::maybestd::io::Error::new(
                            borsh::maybestd::io::ErrorKind::InvalidInput,
                            msg,
                        ));
                    }
                };
                Ok(return_value)
            }
        }
        impl borsh::ser::BorshSerialize for PeriodType {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> core::result::Result<(), borsh::maybestd::io::Error> {
                let variant_idx: u8 = match self {
                    PeriodType::Daily => 0u8,
                    PeriodType::Monthly => 1u8,
                    PeriodType::Quarterly => 2u8,
                    PeriodType::SemiAnnually => 3u8,
                    PeriodType::Yearly => 4u8,
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                match self {
                    PeriodType::Daily => {}
                    PeriodType::Monthly => {}
                    PeriodType::Quarterly => {}
                    PeriodType::SemiAnnually => {}
                    PeriodType::Yearly => {}
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for PeriodType {}
        #[automatically_derived]
        impl ::core::clone::Clone for PeriodType {
            #[inline]
            fn clone(&self) -> PeriodType {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PeriodType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    PeriodType::Daily => ::core::fmt::Formatter::write_str(f, "Daily"),
                    PeriodType::Monthly => ::core::fmt::Formatter::write_str(f, "Monthly"),
                    PeriodType::Quarterly => ::core::fmt::Formatter::write_str(f, "Quarterly"),
                    PeriodType::SemiAnnually => {
                        ::core::fmt::Formatter::write_str(f, "SemiAnnually")
                    }
                    PeriodType::Yearly => ::core::fmt::Formatter::write_str(f, "Yearly"),
                }
            }
        }
        impl ::core::marker::StructuralEq for PeriodType {}
        #[automatically_derived]
        impl ::core::cmp::Eq for PeriodType {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl ::core::marker::StructuralPartialEq for PeriodType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PeriodType {
            #[inline]
            fn eq(&self, other: &PeriodType) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for PeriodType {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state)
            }
        }
        impl Default for PeriodType {
            fn default() -> Self {
                PeriodType::Monthly
            }
        }
        impl PeriodType {
            pub fn from_u32(value: u32) -> Result<PeriodType> {
                match value {
                    1 => Ok(PeriodType::Daily),
                    2 => Ok(PeriodType::Monthly),
                    3 => Ok(PeriodType::Quarterly),
                    4 => Ok(PeriodType::SemiAnnually),
                    5 => Ok(PeriodType::Yearly),
                    _ => Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: err::arg("The period type enumeration is invalid").name(),
                            error_code_number: err::arg("The period type enumeration is invalid")
                                .into(),
                            error_msg: err::arg("The period type enumeration is invalid")
                                .to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/vesting-treasury/src/models/vesting.rs",
                                    line: 289u32,
                                },
                            )),
                            compared_values: None,
                        },
                    )),
                }
            }
        }
        /// Computes the amount of periods in between two dates in the same year.
        /// As an example:
        /// cliff_dt = 15/03/2020
        /// current_dt = 20/09/2020
        ///
        /// First we confirm that current_dt is not before the cliff_dt, otherwise
        /// we return zero periods. Otherwise, we subtract the amount of months between
        /// each date and add 1 period in the curren_dt day is equal or after the
        /// cliff_dt day. In our example the curent day is 20 which is superior or
        /// equal to 15. We therefore count that month as a period, hence why we add 1.
        pub fn compute_periods_from_cliff_to_current_dt(
            cliff_dt: DateTime<Utc>,
            current_dt: DateTime<Utc>,
        ) -> u32 {
            if current_dt.month() < cliff_dt.month() {
                return 0;
            }
            current_dt.month()
                - cliff_dt.month()
                - if current_dt.day() < cliff_dt.day() {
                    1
                } else {
                    0
                }
        }
        /// Computes the amount of periods the cliff_dt datetime till the end of the
        /// year (eoy). Since the cliff_dt datetime day is guaranteed to match the day in
        /// which the vesting schedule started, we know that we can easily subtract the
        /// total number of periods in a year (if monthly this means 12) by the period
        /// refering the to the cliff_dt (i.e. month)
        pub fn compute_periods_from_cliff_to_eoy(cliff_dt: DateTime<Utc>) -> u32 {
            12 - cliff_dt.month()
        }
        /// Computes the amount of periods from the beginning of the current year (boy)
        /// to the current_dt datetime.
        ///
        /// When PeriodType is Monthly the number or periods will be the current_dt
        /// month minus 1 in case the current day is inferior to the cliff_dt day.
        /// Since the cliff_dt datetime day is guaranteed to match the day in
        /// which the vesting schedule started, we we look at the cliff_dt day and
        /// compare it to the current day to infer if we should count or not with the
        /// current period, hence the substrating by 1 means that we are taking our the
        /// current period because this one has not finished.
        pub fn compute_periods_from_boy_to_current_dt(
            cliff_dt: DateTime<Utc>,
            current_dt: DateTime<Utc>,
        ) -> u32 {
            current_dt.month()
                - if current_dt.day() >= cliff_dt.day() {
                    0
                } else {
                    1
                }
        }
        /// Computes the amount of periods in a set of full years, represented by
        /// `delta_years` - 1. In the case the PeriodType is Monthly, we simply
        /// multiply the number of years by 12
        pub fn compute_periods_in_full_years(delta_years: u32) -> u32 {
            (delta_years - 1) * 12
        }
    }
    use crate::prelude::*;
    pub use vesting::*;
    pub struct TokenAmount {
        pub amount: u64,
    }
    impl borsh::de::BorshDeserialize for TokenAmount
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for TokenAmount
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TokenAmount {
        #[inline]
        fn clone(&self) -> TokenAmount {
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TokenAmount {}
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenAmount {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TokenAmount",
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TokenAmount {
        #[inline]
        fn default() -> TokenAmount {
            TokenAmount {
                amount: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::marker::StructuralEq for TokenAmount {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TokenAmount {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u64>;
        }
    }
    impl ::core::marker::StructuralPartialEq for TokenAmount {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TokenAmount {
        #[inline]
        fn eq(&self, other: &TokenAmount) -> bool {
            self.amount == other.amount
        }
        #[inline]
        fn ne(&self, other: &TokenAmount) -> bool {
            self.amount != other.amount
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for TokenAmount {
        #[inline]
        fn cmp(&self, other: &TokenAmount) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.amount, &other.amount)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for TokenAmount {
        #[inline]
        fn partial_cmp(
            &self,
            other: &TokenAmount,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.amount, &other.amount)
        }
    }
    pub struct TimeStamp {
        pub time: i64,
    }
    impl borsh::de::BorshDeserialize for TimeStamp
    where
        i64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                time: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for TimeStamp
    where
        i64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.time, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TimeStamp {
        #[inline]
        fn clone(&self) -> TimeStamp {
            let _: ::core::clone::AssertParamIsClone<i64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TimeStamp {}
    #[automatically_derived]
    impl ::core::fmt::Debug for TimeStamp {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "TimeStamp", "time", &&self.time)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TimeStamp {
        #[inline]
        fn default() -> TimeStamp {
            TimeStamp {
                time: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::marker::StructuralEq for TimeStamp {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TimeStamp {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i64>;
        }
    }
    impl ::core::marker::StructuralPartialEq for TimeStamp {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TimeStamp {
        #[inline]
        fn eq(&self, other: &TimeStamp) -> bool {
            self.time == other.time
        }
        #[inline]
        fn ne(&self, other: &TimeStamp) -> bool {
            self.time != other.time
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for TimeStamp {
        #[inline]
        fn cmp(&self, other: &TimeStamp) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.time, &other.time)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for TimeStamp {
        #[inline]
        fn partial_cmp(&self, other: &TimeStamp) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.time, &other.time)
        }
    }
    pub struct Slot {
        pub slot: u64,
    }
    impl borsh::de::BorshDeserialize for Slot
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                slot: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for Slot
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.slot, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Slot {
        #[inline]
        fn clone(&self) -> Slot {
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Slot {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Slot {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Slot", "slot", &&self.slot)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Slot {
        #[inline]
        fn default() -> Slot {
            Slot {
                slot: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::marker::StructuralEq for Slot {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Slot {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u64>;
        }
    }
    impl ::core::marker::StructuralPartialEq for Slot {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Slot {
        #[inline]
        fn eq(&self, other: &Slot) -> bool {
            self.slot == other.slot
        }
        #[inline]
        fn ne(&self, other: &Slot) -> bool {
            self.slot != other.slot
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Slot {
        #[inline]
        fn cmp(&self, other: &Slot) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.slot, &other.slot)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Slot {
        #[inline]
        fn partial_cmp(&self, other: &Slot) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.slot, &other.slot)
        }
    }
    pub struct Permillion {
        /// 1% = 10_000
        pub permillion: u64,
    }
    impl borsh::de::BorshDeserialize for Permillion
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                permillion: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl borsh::ser::BorshSerialize for Permillion
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.permillion, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Permillion {
        #[inline]
        fn clone(&self) -> Permillion {
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Permillion {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Permillion {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Permillion",
                "permillion",
                &&self.permillion,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Permillion {
        #[inline]
        fn default() -> Permillion {
            Permillion {
                permillion: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::marker::StructuralEq for Permillion {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Permillion {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u64>;
        }
    }
    impl ::core::marker::StructuralPartialEq for Permillion {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Permillion {
        #[inline]
        fn eq(&self, other: &Permillion) -> bool {
            self.permillion == other.permillion
        }
        #[inline]
        fn ne(&self, other: &Permillion) -> bool {
            self.permillion != other.permillion
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Permillion {
        #[inline]
        fn cmp(&self, other: &Permillion) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.permillion, &other.permillion)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Permillion {
        #[inline]
        fn partial_cmp(&self, other: &Permillion) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.permillion, &other.permillion)
        }
    }
    impl TokenAmount {
        pub fn new(amount: u64) -> Self {
            Self { amount }
        }
        pub fn max_value() -> Self {
            Self {
                amount: std::u64::MAX,
            }
        }
    }
    impl TimeStamp {
        pub fn new(time: i64) -> Self {
            Self { time }
        }
        pub fn new_dt(date: Date<Utc>) -> Self {
            let time = date.and_hms_nano(0, 0, 0, 0).timestamp();
            Self { time }
        }
    }
    impl Slot {
        pub fn new(slot: u64) -> Self {
            Self { slot }
        }
        pub fn current() -> Result<Self> {
            Ok(Self {
                slot: Clock::get()?.slot,
            })
        }
    }
    impl Permillion {
        pub fn from_percent(percent: u64) -> Self {
            Self {
                permillion: percent.checked_mul(10_000).unwrap(),
            }
        }
    }
    impl From<TokenAmount> for Decimal {
        fn from(tokens: TokenAmount) -> Self {
            Decimal::from(tokens.amount)
        }
    }
    impl From<Permillion> for Decimal {
        fn from(permillion: Permillion) -> Self {
            Decimal::from_permillion(permillion.permillion)
        }
    }
    impl From<u64> for TokenAmount {
        fn from(amount: u64) -> Self {
            Self { amount }
        }
    }
}
pub mod prelude {
    pub use crate::endpoints;
    pub use crate::err::{self, TreasuryError};
    pub use crate::models::*;
    pub use crate::time::{self, *};
    pub use anchor_lang::prelude::*;
    pub use chrono::prelude::*;
    pub use decimal::{Decimal, TryAdd, TryDiv, TryMul, TryRound, TrySub};
}
pub mod time {
    pub mod timedelta {
        //! Contains utility functions for shifting Date objects.
        use chrono::Datelike;
        /// Returns true if the year is a leap-year, as naively defined in the Gregorian calendar.
        #[inline]
        pub fn is_leap_year(year: i32) -> bool {
            year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
        }
        #[inline]
        fn normalise_day(year: i32, month: u32, day: u32) -> u32 {
            if day <= 28 {
                day
            } else if month == 2 {
                28 + is_leap_year(year) as u32
            } else if day == 31 && (month == 4 || month == 6 || month == 9 || month == 11) {
                30
            } else {
                day
            }
        }
        /// Shift a date by the given number of months.
        /// Ambiguous month-ends are shifted backwards as necessary.
        pub fn shift_months<D: Datelike>(date: D, months: i32) -> D {
            let mut year = date.year() + (date.month() as i32 + months) / 12;
            let mut month = (date.month() as i32 + months) % 12;
            let mut day = date.day();
            if month < 1 {
                year -= 1;
                month += 12;
            }
            day = normalise_day(year, month as u32, day);
            if day <= 28 {
                date.with_day(day)
                    .unwrap()
                    .with_month(month as u32)
                    .unwrap()
                    .with_year(year)
                    .unwrap()
            } else {
                date.with_day(1)
                    .unwrap()
                    .with_month(month as u32)
                    .unwrap()
                    .with_year(year)
                    .unwrap()
                    .with_day(day)
                    .unwrap()
            }
        }
        /// Shift a date by the given number of years.
        /// Ambiguous month-ends are shifted backwards as necessary.
        pub fn shift_years<D: Datelike>(date: D, years: i32) -> D {
            shift_months(date, years * 12)
        }
        /// Shift the date to have the given day.  Returns None if the day is not in the range 1-31.
        ///
        /// Ambiguous month-ends are shifted backwards as necessary.
        pub fn with_day<D: Datelike>(date: D, day: u32) -> Option<D> {
            if day == 0 || day > 31 {
                None
            } else {
                date.with_day(normalise_day(date.year(), date.month(), day))
            }
        }
        /// Shift the date to have the given month. Returns None if the month is out of range.
        ///
        /// Ambiguous month-ends are shifted backwards as necessary.
        pub fn with_month<D: Datelike>(date: D, month: u32) -> Option<D> {
            if month == 0 || month > 12 {
                None
            } else {
                let delta = month as i32 - date.month() as i32;
                Some(shift_months(date, delta))
            }
        }
        /// Shift the date to have the given year.
        ///
        /// Ambiguous month-ends are shifted backwards as necessary.
        pub fn with_year<D: Datelike>(date: D, year: i32) -> D {
            let delta = year - date.year();
            shift_years(date, delta)
        }
    }
    pub use timedelta::*;
}
use crate::endpoints::*;
use crate::prelude::*;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        218u8, 7u8, 92u8, 178u8, 255u8, 94u8, 198u8, 129u8, 118u8, 19u8, 222u8, 83u8, 11u8, 105u8,
        42u8, 135u8, 53u8, 71u8, 119u8, 105u8, 218u8, 71u8, 67u8, 12u8, 189u8, 129u8, 84u8, 51u8,
        92u8, 74u8, 131u8, 39u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
use self::vesting_treasury::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data).map_err(|e| {
        e.log();
        e.into()
    })
}
fn try_entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct VestingTreasury;
    #[automatically_derived]
    impl ::core::clone::Clone for VestingTreasury {
        #[inline]
        fn clone(&self) -> VestingTreasury {
            VestingTreasury
        }
    }
    impl anchor_lang::Id for VestingTreasury {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>::<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [195, 30, 184, 253, 77, 154, 187, 66] => {
            __private::__global::create_vesting_schedule(program_id, accounts, ix_data)
        }
        [139, 99, 205, 250, 192, 213, 215, 89] => {
            __private::__global::change_vestee_wallet(program_id, accounts, ix_data)
        }
        [31, 140, 92, 231, 182, 206, 70, 107] => {
            __private::__global::update_vested_tokens(program_id, accounts, ix_data)
        }
        [77, 90, 197, 75, 78, 122, 74, 244] => {
            __private::__global::fund_vesting_vault(program_id, accounts, ix_data)
        }
        [73, 210, 194, 163, 55, 82, 114, 85] => {
            __private::__global::withdraw_vested_tokens(program_id, accounts, ix_data)
        }
        [53, 177, 56, 104, 70, 183, 187, 179] => {
            __private::__global::close_vesting_schedule(program_id, accounts, ix_data)
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn create_vesting_schedule(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CreateVestingSchedule");
            let ix = instruction::CreateVestingSchedule::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateVestingSchedule {
                vesting_amount,
                start_ts,
                cliff_periods,
                total_periods,
                period_type,
            } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = CreateVestingSchedule::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result = vesting_treasury::create_vesting_schedule(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                vesting_amount,
                start_ts,
                cliff_periods,
                total_periods,
                period_type,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn change_vestee_wallet(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: ChangeVesteeWallet");
            let ix = instruction::ChangeVesteeWallet::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::ChangeVesteeWallet = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = ChangeVesteeWallet::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result =
                vesting_treasury::change_vestee_wallet(anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn update_vested_tokens(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: UpdateVestedTokens");
            let ix = instruction::UpdateVestedTokens::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::UpdateVestedTokens = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = UpdateVestedTokens::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result =
                vesting_treasury::update_vested_tokens(anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn fund_vesting_vault(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: FundVestingVault");
            let ix = instruction::FundVestingVault::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::FundVestingVault { funding_amount } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = FundVestingVault::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result = vesting_treasury::fund_vesting_vault(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                funding_amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn withdraw_vested_tokens(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: WithdrawVestedTokens");
            let ix = instruction::WithdrawVestedTokens::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::WithdrawVestedTokens { withdraw_amount } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = WithdrawVestedTokens::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result = vesting_treasury::withdraw_vested_tokens(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                withdraw_amount,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn close_vesting_schedule(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CloseVestingSchedule");
            let ix = instruction::CloseVestingSchedule::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CloseVestingSchedule = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = CloseVestingSchedule::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result =
                vesting_treasury::close_vesting_schedule(anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ))?;
            accounts.exit(program_id)
        }
    }
}
pub mod vesting_treasury {
    use super::*;
    pub fn create_vesting_schedule(
        ctx: Context<CreateVestingSchedule>,
        vesting_amount: TokenAmount,
        start_ts: TimeStamp,
        cliff_periods: u64,
        total_periods: u64,
        period_type: u32,
    ) -> Result<()> {
        endpoints::create_vesting_schedule::handle(
            ctx,
            vesting_amount,
            start_ts,
            cliff_periods,
            total_periods,
            period_type,
        )
    }
    pub fn change_vestee_wallet(ctx: Context<ChangeVesteeWallet>) -> Result<()> {
        endpoints::change_vestee_wallet::handle(ctx)
    }
    pub fn update_vested_tokens(ctx: Context<UpdateVestedTokens>) -> Result<()> {
        endpoints::update_vested_tokens::handle(ctx)
    }
    pub fn fund_vesting_vault(
        ctx: Context<FundVestingVault>,
        funding_amount: TokenAmount,
    ) -> Result<()> {
        endpoints::fund_vesting_vault::handle(ctx, funding_amount)
    }
    pub fn withdraw_vested_tokens(
        ctx: Context<WithdrawVestedTokens>,
        withdraw_amount: TokenAmount,
    ) -> Result<()> {
        endpoints::withdraw_vested_tokens::handle(ctx, withdraw_amount)
    }
    pub fn close_vesting_schedule(ctx: Context<CloseVestingSchedule>) -> Result<()> {
        endpoints::close_vesting_schedule::handle(ctx)
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct CreateVestingSchedule {
        pub vesting_amount: TokenAmount,
        pub start_ts: TimeStamp,
        pub cliff_periods: u64,
        pub total_periods: u64,
        pub period_type: u32,
    }
    impl borsh::ser::BorshSerialize for CreateVestingSchedule
    where
        TokenAmount: borsh::ser::BorshSerialize,
        TimeStamp: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u32: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.vesting_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.start_ts, writer)?;
            borsh::BorshSerialize::serialize(&self.cliff_periods, writer)?;
            borsh::BorshSerialize::serialize(&self.total_periods, writer)?;
            borsh::BorshSerialize::serialize(&self.period_type, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateVestingSchedule
    where
        TokenAmount: borsh::BorshDeserialize,
        TimeStamp: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u32: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                vesting_amount: borsh::BorshDeserialize::deserialize(buf)?,
                start_ts: borsh::BorshDeserialize::deserialize(buf)?,
                cliff_periods: borsh::BorshDeserialize::deserialize(buf)?,
                total_periods: borsh::BorshDeserialize::deserialize(buf)?,
                period_type: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for CreateVestingSchedule {
        fn data(&self) -> Vec<u8> {
            let mut d = [195, 30, 184, 253, 77, 154, 187, 66].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct ChangeVesteeWallet;
    impl borsh::ser::BorshSerialize for ChangeVesteeWallet {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ChangeVesteeWallet {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for ChangeVesteeWallet {
        fn data(&self) -> Vec<u8> {
            let mut d = [139, 99, 205, 250, 192, 213, 215, 89].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct UpdateVestedTokens;
    impl borsh::ser::BorshSerialize for UpdateVestedTokens {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateVestedTokens {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for UpdateVestedTokens {
        fn data(&self) -> Vec<u8> {
            let mut d = [31, 140, 92, 231, 182, 206, 70, 107].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct FundVestingVault {
        pub funding_amount: TokenAmount,
    }
    impl borsh::ser::BorshSerialize for FundVestingVault
    where
        TokenAmount: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.funding_amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for FundVestingVault
    where
        TokenAmount: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                funding_amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for FundVestingVault {
        fn data(&self) -> Vec<u8> {
            let mut d = [77, 90, 197, 75, 78, 122, 74, 244].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct WithdrawVestedTokens {
        pub withdraw_amount: TokenAmount,
    }
    impl borsh::ser::BorshSerialize for WithdrawVestedTokens
    where
        TokenAmount: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.withdraw_amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WithdrawVestedTokens
    where
        TokenAmount: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                withdraw_amount: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for WithdrawVestedTokens {
        fn data(&self) -> Vec<u8> {
            let mut d = [73, 210, 194, 163, 55, 82, 114, 85].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CloseVestingSchedule;
    impl borsh::ser::BorshSerialize for CloseVestingSchedule {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CloseVestingSchedule {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for CloseVestingSchedule {
        fn data(&self) -> Vec<u8> {
            let mut d = [53, 177, 56, 104, 70, 183, 187, 179].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_fund_vesting_vault::*;
    pub use crate::__client_accounts_close_vesting_schedule::*;
    pub use crate::__client_accounts_withdraw_vested_tokens::*;
    pub use crate::__client_accounts_update_vested_tokens::*;
    pub use crate::__client_accounts_change_vestee_wallet::*;
    pub use crate::__client_accounts_create_vesting_schedule::*;
}
