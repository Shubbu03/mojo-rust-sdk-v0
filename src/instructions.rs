use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::{pubkey, Pubkey};
use solana_sdk_ids::sysvar::rent::ID as RENT_SYSVAR_ID;
use solana_system_interface::program::ID as SYSTEM_PROGRAM_ID;

use crate::{
    constants::PROGRAM_ID,
    mojo_types::{GenIxHandler, MojoInstructions},
};

/// The delegation program ID.
pub const DELEGATION_PROGRAM_ID: Pubkey = pubkey!("DELeGGvXpWV2fqJUhqcF5ZSYMS4JTLjteaAMARRSaeSh");

pub const EU_VALIDATOR: Pubkey = pubkey!("MEUGGrYPxKk17hCr7wpT6s8dtNokZj5U2L57vjYMS8e");

/// The magic program ID.
pub const MAGIC_PROGRAM_ID: Pubkey = pubkey!("Magic11111111111111111111111111111111111111");

/// The magic context ID.
pub const MAGIC_CONTEXT_ID: Pubkey = pubkey!("MagicContext1111111111111111111111111111111");

///
/// The seed of the authority account PDA.
pub const DELEGATION_RECORD: &[u8] = b"delegation";

/// The account to store the delegated account seeds.
pub const DELEGATION_METADATA: &[u8] = b"delegation-metadata";

/// The seed of the buffer account PDA.
pub const BUFFER: &[u8] = b"buffer";

/// The seed of the committed state PDA.
pub const COMMIT_STATE: &[u8] = b"state-diff";

/// The seed of a commit state record PDA.
pub const COMMIT_RECORD: &[u8] = b"commit-state-record";

/// The discriminator for the external undelegate instruction.
pub const EXTERNAL_UNDELEGATE_DISCRIMINATOR: [u8; 8] = [196, 28, 41, 206, 48, 37, 51, 167];

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

pub fn delegate_account_ix(
    payer: Pubkey,
    account: Pubkey,
    seed_hash: [u8; 32],
    state_data: &[u8],
) -> Instruction {
    let mut handler = GenIxHandler::new((state_data.len() as u64).to_le_bytes());
    handler.seeds.copy_from_slice(&seed_hash);

    let buffer_account = Pubkey::find_program_address(&[BUFFER, account.as_ref()], &PROGRAM_ID).0;

    // Derive delegation_record PDA: ["delegation", account_pubkey]
    let delegation_record =
        Pubkey::find_program_address(&[b"delegation", account.as_ref()], &DELEGATION_PROGRAM_ID).0;

    let delegation_metadata = Pubkey::find_program_address(
        &[b"delegation-metadata", account.as_ref()],
        &DELEGATION_PROGRAM_ID,
    )
    .0;

    let data = encode_instruction(MojoInstructions::DelegateAccount, &handler, state_data);

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(payer, true),                  // creator/payer
            AccountMeta::new(account, false),               // account to delegate
            AccountMeta::new(PROGRAM_ID, false),            // owner program
            AccountMeta::new(buffer_account, false), // buffer PDA (created via invoke_signed)
            AccountMeta::new(delegation_record, false), // delegation record
            AccountMeta::new(delegation_metadata, false), // delegation metadata
            AccountMeta::new(SYSTEM_PROGRAM_ID, false), // system program
            AccountMeta::new(DELEGATION_PROGRAM_ID, false), // Delegation Program ID
            AccountMeta::new(EU_VALIDATOR, false),   // Europe Validator
        ],
        data: data,
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

    let data = encode_instruction(
        MojoInstructions::UpdateDelegatedAccount,
        &handler,
        state_data,
    );

    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(world, false),
            AccountMeta::new_readonly(MAGIC_CONTEXT_ID, false),
            AccountMeta::new_readonly(MAGIC_PROGRAM_ID, false),
        ],
        data,
    }
}
