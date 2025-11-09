use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_sdk_ids::sysvar::rent::ID as RENT_SYSVAR_ID;
use solana_system_interface::program::ID as SYSTEM_PROGRAM_ID;

use crate::{
    constants::PROGRAM_ID,
    mojo_types::{GenIxHandler, MojoInstructions},
};

fn encode_instruction(
    discriminator: MojoInstructions,
    handler: &GenIxHandler,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(1 + GenIxHandler::LEN + payload.len());
    bytes.push(discriminator as u8);

    let handler_bytes = handler.to_bytes();
    bytes.extend_from_slice(&handler_bytes);
    bytes.extend_from_slice(payload);
    bytes
}

pub fn create_world_ix(
    payer: Pubkey,
    world: Pubkey,
    seed_hash: [u8; 32],
    state_data: &[u8],
) -> Instruction {
    let mut handler = GenIxHandler::new((state_data.len() as u64).to_le_bytes());
    handler.seeds.copy_from_slice(&seed_hash);

    let data = encode_instruction(MojoInstructions::CreateAccount, &handler, state_data);
    let rent_pubkey = Pubkey::new_from_array(RENT_SYSVAR_ID.to_bytes());

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(world, false),
            AccountMeta::new(SYSTEM_PROGRAM_ID, false),
            AccountMeta::new(rent_pubkey, false),
        ],
        data,
    }
}

pub fn write_to_world_ix(
    payer: Pubkey,
    world: Pubkey,
    seed_hash: [u8; 32],
    state_data: &[u8],
) -> Instruction {
    let mut handler = GenIxHandler::new((state_data.len() as u64).to_le_bytes());
    handler.seeds.copy_from_slice(&seed_hash);

    let data = encode_instruction(MojoInstructions::UpdateDelegatedAccount, &handler, state_data);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(world, false),
        ],
        data,
    }
}
