- Solana v1.9.18
- Anchor v0.24.2
- [Code coverage][project-code-coverage]

# Install

1. `$ yarn` fetches test dependencies
2. `$ npm run build` builds all programs

# Vesting Treasury
- [Rust docs][amm-rust-docs]
- [Changelog][amm-changelog]
- [`inser pubkey here` is dev program][amm-dev-solscan]

This program functions as Aldrin's vesting treasury manager. The main intent of this program is to allow Aldrin to programatically instantiate vesting schedules on the blockchain. The setup is nevertheless permissionless, so anyone can create a vesting schedule for any arbitrary mint and act as the administrator of such vesting. Each vesting schedule has a target wallet which the funds will be withdrawn to once available. The administrator of the vesting schedule can change the target wallet, which is useful if such wallet becomes compromised.


## Data model

At the core of this program's data model is the `Vesting` account, which has the following fields:

| Field                  | Type          | Description |
------- | ----------- |
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

As time passes by, more tokens get vested as per the schedule. Hence the purpose of this endpoint is to update the field `cumulative_vested` in the `Vesting` account. We update it according to the following logic:

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


The $`\Delta p`$ or `Δperiods` is computed depending on the `PeriodType` but essentially is the amount of periods that have passed since the cliff date.

## Add your files

- [ ] [Create](https://docs.gitlab.com/ee/user/project/repository/web_editor.html#create-a-file) or [upload](https://docs.gitlab.com/ee/user/project/repository/web_editor.html#upload-a-file) files
- [ ] [Add files using the command line](https://docs.gitlab.com/ee/gitlab-basics/add-file.html#add-a-file-using-the-command-line) or push an existing Git repository with the following command:

```
cd existing_repo
git remote add origin https://gitlab.com/crypto_project/defi/team-vesting.git
git branch -M main
git push -uf origin main
```

## Integrate with your tools

- [ ] [Set up project integrations](https://gitlab.com/crypto_project/defi/team-vesting/-/settings/integrations)

## Collaborate with your team

- [ ] [Invite team members and collaborators](https://docs.gitlab.com/ee/user/project/members/)
- [ ] [Create a new merge request](https://docs.gitlab.com/ee/user/project/merge_requests/creating_merge_requests.html)
- [ ] [Automatically close issues from merge requests](https://docs.gitlab.com/ee/user/project/issues/managing_issues.html#closing-issues-automatically)
- [ ] [Enable merge request approvals](https://docs.gitlab.com/ee/user/project/merge_requests/approvals/)
- [ ] [Automatically merge when pipeline succeeds](https://docs.gitlab.com/ee/user/project/merge_requests/merge_when_pipeline_succeeds.html)

## Test and Deploy

Use the built-in continuous integration in GitLab.

- [ ] [Get started with GitLab CI/CD](https://docs.gitlab.com/ee/ci/quick_start/index.html)
- [ ] [Analyze your code for known vulnerabilities with Static Application Security Testing(SAST)](https://docs.gitlab.com/ee/user/application_security/sast/)
- [ ] [Deploy to Kubernetes, Amazon EC2, or Amazon ECS using Auto Deploy](https://docs.gitlab.com/ee/topics/autodevops/requirements.html)
- [ ] [Use pull-based deployments for improved Kubernetes management](https://docs.gitlab.com/ee/user/clusters/agent/)
- [ ] [Set up protected environments](https://docs.gitlab.com/ee/ci/environments/protected_environments.html)

***

# Editing this README

When you're ready to make this README your own, just edit this file and use the handy template below (or feel free to structure it however you want - this is just a starting point!). Thank you to [makeareadme.com](https://www.makeareadme.com/) for this template.

## Suggestions for a good README
Every project is different, so consider which of these sections apply to yours. The sections used in the template are suggestions for most open source projects. Also keep in mind that while a README can be too long and detailed, too long is better than too short. If you think your README is too long, consider utilizing another form of documentation rather than cutting out information.

## Name
Choose a self-explaining name for your project.

## Description
Let people know what your project can do specifically. Provide context and add a link to any reference visitors might be unfamiliar with. A list of Features or a Background subsection can also be added here. If there are alternatives to your project, this is a good place to list differentiating factors.

## Badges
On some READMEs, you may see small images that convey metadata, such as whether or not all the tests are passing for the project. You can use Shields to add some to your README. Many services also have instructions for adding a badge.

## Visuals
Depending on what you are making, it can be a good idea to include screenshots or even a video (you'll frequently see GIFs rather than actual videos). Tools like ttygif can help, but check out Asciinema for a more sophisticated method.

## Installation
Within a particular ecosystem, there may be a common way of installing things, such as using Yarn, NuGet, or Homebrew. However, consider the possibility that whoever is reading your README is a novice and would like more guidance. Listing specific steps helps remove ambiguity and gets people to using your project as quickly as possible. If it only runs in a specific context like a particular programming language version or operating system or has dependencies that have to be installed manually, also add a Requirements subsection.

## Usage
Use examples liberally, and show the expected output if you can. It's helpful to have inline the smallest example of usage that you can demonstrate, while providing links to more sophisticated examples if they are too long to reasonably include in the README.

## Support
Tell people where they can go to for help. It can be any combination of an issue tracker, a chat room, an email address, etc.

## Roadmap
If you have ideas for releases in the future, it is a good idea to list them in the README.

## Contributing
State if you are open to contributions and what your requirements are for accepting them.

For people who want to make changes to your project, it's helpful to have some documentation on how to get started. Perhaps there is a script that they should run or some environment variables that they need to set. Make these steps explicit. These instructions could also be useful to your future self.

You can also document commands to lint the code or run tests. These steps help to ensure high code quality and reduce the likelihood that the changes inadvertently break something. Having instructions for running tests is especially helpful if it requires external setup, such as starting a Selenium server for testing in a browser.

## Authors and acknowledgment
Show your appreciation to those who have contributed to the project.

## License
For open source projects, say how it is licensed.

## Project status
If you have run out of energy or time for your project, put a note at the top of the README saying that development has slowed down or stopped completely. Someone may choose to fork your project or volunteer to step in as a maintainer or owner, allowing your project to keep going. You can also make an explicit request for maintainers.
