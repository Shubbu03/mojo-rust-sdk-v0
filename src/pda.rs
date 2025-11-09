use sha2::{Digest, Sha256};
use solana_pubkey::Pubkey;

use crate::constants::PROGRAM_ID;

fn compute_seed_hash(chunks: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for chunk in chunks {
        hasher.update(chunk);
    }
    hasher.finalize().into()
}

pub fn world_seed_hash(owner: &Pubkey, name: &str) -> [u8; 32] {
    compute_seed_hash(&[b"world", name.as_bytes(), owner.as_ref()])
}

pub fn state_seed_hash(world: &Pubkey, state_name: &str, owner: &Pubkey) -> [u8; 32] {
    compute_seed_hash(&[b"state", world.as_ref(), state_name.as_bytes(), owner.as_ref()])
}

pub fn find_world_pda(owner: &Pubkey, name: &str) -> (Pubkey, u8) {
    let seed_hash = world_seed_hash(owner, name);
    Pubkey::find_program_address(&[seed_hash.as_ref(), owner.as_ref()], &PROGRAM_ID)
}

pub fn find_state_pda(world: &Pubkey, state_name: &str, owner: &Pubkey) -> (Pubkey, u8) {
    let seed_hash = state_seed_hash(world, state_name, owner);
    Pubkey::find_program_address(&[seed_hash.as_ref(), owner.as_ref()], &PROGRAM_ID)
}
