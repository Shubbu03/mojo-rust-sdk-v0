use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

use crate::constants::PROGRAM_ID;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum WorldIxs {
    CreateWorld { name: String },
    WriteToWorld { data: Vec<u8> },
}

pub fn create_world_ix(payer: Pubkey, world: Pubkey, name: String) -> Instruction {
    let data = to_vec(&WorldIxs::CreateWorld { name }).unwrap();

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(world, false),
        ],
        data,
    }
}

pub fn write_to_world_ix(payer: Pubkey, world: Pubkey, data_bytes: Vec<u8>) -> Instruction {
    let data = to_vec(&WorldIxs::WriteToWorld { data: data_bytes }).unwrap();

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(world, false),
        ],
        data,
    }
}
