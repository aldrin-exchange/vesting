# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a
Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2022-08-11
### Changed

- Removed `clock` account from account context in endpoint 
  `update_vested_tokens`

## [0.7.0] - 2022-08-11
### Added

- Calculation logic for daily vestings added. The contract now allows for
  vesting accounts with period_type of `PeriodType::Daily`

## [0.6.0] - 2022-08-04

### Added

- Endpoint `close_vesting_schedule`

## [0.5.0] - 2022-07-29

### Added

- Endpoint `withdraw_vested_tokens`
  
### Changed

- Renamed the following fields in account struct `Vesting`:
  - `total_vesting_amount` to `total_vesting`
  - `cumulative_vested_amount` to `cumulative_vested`
  - `cumulative_withdrawn_amount` to `cumulative_withdrawn`

## [0.4.0] - 2022-07-29

### Added

- Endpoint `fund_vesting_vault`

## [0.3.0] - 2022-07-27

### Added

- Endpoint `update_vested_tokens`
- Method `update_vested_tokens` in struct `Vesting` which contains
  core logic for the endpoint
- Method `update_unfunded_liability` in struct `Vesting`
- Chrono external crate to handle datetime calculations

## [0.2.0] - 2022-07-27

### Added

- Endpoint `change_vestee_wallet`

## [0.1.0] - 2022-07-27

### Added

- Vesting account model
- Enum `PeriodType` used in vesting field `period_type`
- Decimal external crate
- Endpoint `create_vesting_schedule`