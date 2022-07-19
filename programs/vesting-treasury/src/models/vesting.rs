use crate::prelude::*;

#[derive(Default, Debug)]
#[account]
pub struct Vesting {
    pub admin: Pubkey,
}

impl Vesting {
    pub const VAULT_PREFIX: &'static [u8; 5] = b"vault";
    pub const SIGNER_PDA_PREFIX: &'static [u8; 6] = b"signer";

    pub fn space() -> usize {
        //TODO: this is a placeholder for the timebeing
        8_usize
    }
}
