- Solana v1.9.18
- Anchor v0.24.2
- [Code coverage][project-code-coverage]

# Install

1. `$ yarn` fetches test dependencies
2. `$ npm run build` builds all programs

# Tests

1. `$ cargo test` to run Rust unit and integration tests
2. `$ ./bin/test.sh` to run Typescript integration tests


# Vesting Treasury
- [Rust docs][treasury-rust-docs]
- [Changelog][treasury-changelog]
- [`inser pubkey here` is dev program][treasury-dev-solscan]

This program functions as Aldrin's vesting treasury manager. The main intent of this program is to allow Aldrin to programatically instantiate vesting schedules on the blockchain. The setup is nevertheless permissionless, so anyone can create a vesting schedule for any arbitrary mint and act as the administrator of such vesting. Each vesting schedule has a target wallet which the funds will be withdrawn to once available. The administrator of the vesting schedule can change the target wallet, which is useful if such wallet becomes compromised.


## Data model

At the core of this program's data model is the `Vesting` account, which has the following fields:

| Field                  | Type          | Description |
| ---------------------- | ------------- | ----------- |
| `admin`                | `Pubkey`      | The authority of this Vesting account |
| `vestee_wallet`        | `Pubkey`      | The vestee wallet of this Vesting account, that should receive the vested funds |
| `mint`                 | `Pubkey`      | The mint of the SPL token locked up |
| `vault`                | `Pubkey`      | Address of the account's token vault |
| `total_vesting`        | `TokenAmount` | The total amount that will vest over time |
| `cumulative_vested`    | `TokenAmount` | Cumulative amount that vested thus far |
| `cumulative_withdrawn` | `TokenAmount` | Cumulative amount withdrawn thus far |
| `vault_balance`        | `TokenAmount` | Current amount available in the vesting vault |
| `unfunded_liability`   | `TokenAmount` | The unfunded liability is the amount of vested tokens that a user is already allowed to withdraw but are still not available in the vesting vault, therefore constituting a liability on behalf of the funder |
| `start_ts`             | `TimeStamp`   | The start time in Unix Timestamp of the vesting period |
| `total_periods`        | `u64`         | The amount of periods in total in the vesting schedule, where a period represents a different timestamp depending on the period_type |
| `cliff_periods`        | `u64`         | The amount of periods in the cliff part of the schedule, where a period represents a different timestamp  |
| `period_type`          | `PeriodType`  | The type of period (i.e. Monthly, Yearly, etc.) of the vesting schedule. This is required for computing vesting schedules depending on different base periods |

Where `TokenAmount` is a struct with the field `amount` as a `u64`, and `TimeStamp` is a struct with the field `time` as a `i64`. The type `PeriodType` is an Enum with the following enumerations:

```
pub enum PeriodType {
    Daily,
    Monthly,
    Quarterly,
    SemiAnnually,
    Yearly,
}
```

## Endpoints

The program has the following endpoints:
- `create_vesting_schedule`
- `change_vestee_wallet`
- `update_vested_tokens`
- `fund_vesting_vault`
- `withdraw_vested_tokens`
- `close_vesting_schedule`


### Create Vesting Schedule

The endpoint `create_vesting_schedule` has the following input arguments:

- `vesting_amount: TokenAmount`
- `start_ts: TimeStamp`
- `cliff_periods: u64`
- `total_periods: u64`
- `period_type: u64`

Note: Period type is inputed as a u64 which will be converted to the PeriodType enum as follows:

| u64 value | PeriodType     |
| --------- | -------------- |
| 1         | `Daily`        |
| 2         | `Monthly`      |
| 3         | `Quarterly`    |
| 4         | `SemiAnnually` |
| 5         | `Yearly`       |

In the current program version, only the types `Daily` and `Monthly` are allowed.


### Change Vestee Wallet

The purpose of this endpoint is the change the target wallet in the vesting account.


### Update Vested Tokens

As time passes by, more tokens get vested as per the schedule. Hence the purpose of this permissionless endpoint is to update the field `cumulative_vested` in the `Vesting` account. We update it according to the following logic:

The schedule is composed by two periods, a period of cliff versting in which the tokens are vested only at the end of the cliff date, and a period in which the vesting occurs linearly over time (discrete over the period type).

Let $`p_c`$ be the `cliff_periods` and $`t_s`$ be the `start_ts`. We calculate cliff date $`t_c`$ as:

```math
t_c = t_s + p_c
```

If the current date is before the cliff date, then there are no vested tokens and therefore the logic ends.

Whenever the current date surpasses the cliff date we use the following logic to compute the vested tokens:

```math
V_{cum} = \frac{p_c + \Delta p}{p_T} V_T
```

where $`p_T`$ is `total_periods` in the vesting and $`V_T`$ is `total_vesting`.


The $`\Delta p`$ or `delta_periods` is computed depending on the `PeriodType` but essentially is the amount of periods that have passed since the cliff date.


Once the field `cumulative_vested` is updated, the endpoint logic will update the field `unfunded_liability` which corresponds to the amount of tokens vested that are still not available in the `vesting_vault`. To fund the `vesting_vault` we call the endpoint `fund_vesting_vault`.

### Fund Vesting Vault

This endpoint is used to transfer tokens to the `vesting_vault` and to update the `unfunded_liability` field in the `Vesting` account. This endpoint is permissionless, so technically any account with the token balance of the right mint can call this endpoint and send the tokens to the `vesting_vault`. The endpoint accepts the argument `funding_amount` which is of type `TokenAmount`.

### Withdraw Vested Tokens

Upon calling this endpoint the vested tokens that are available in the `vesting_vault` will be transferred to the target wallet and the field `cumulative_withdrawn`, and `vault_balance` will be updated. The endpoint accepts the argument `withdraw_amount` which is of type `TokenAmount`. If this amount exceed the current amount vested or the current amount available in the `vesting_vault`, the program will return an error.


<!-- List of References -->

[project-code-coverage]: https://crypto_project.gitlab.io/defi/team_vesting/coverage
[treasury-rust-docs]: https://crypto_project.gitlab.io/defi/team_vesting/team_vesting
[treasury-changelog]: https://crypto_project.gitlab.io/defi/team_vesting/team_vesing.changelog.html
[treasury-dev-solscan]: https://solscan.io/account/dAMMP3unWqb4u2La1xczx6JSAZsGByo9amHgzkVY7FG
