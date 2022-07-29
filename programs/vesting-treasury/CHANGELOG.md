# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a
Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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