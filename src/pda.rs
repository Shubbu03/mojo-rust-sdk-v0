use solana_pubkey::Pubkey;

use crate::constants::PROGRAM_ID;

pub fn find_world_pda(owner: &Pubkey, name: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"world", owner.as_ref(), name.as_bytes()], &PROGRAM_ID)
}
