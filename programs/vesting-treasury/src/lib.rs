pub mod endpoints;
pub mod err;
pub mod models;
pub mod prelude;

use crate::endpoints::*;
use crate::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vesting_treasury {
    use super::*;

    pub fn create_vesting_schedule(ctx: Context<CreateVestingSchedule>) -> Result<()> {
        endpoints::create_vesting_schedule::handle(ctx)
    }
}
