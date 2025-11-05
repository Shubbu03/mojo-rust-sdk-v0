use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WorldAccount {
    pub owner: Pubkey,
    pub name: String,
    pub data: Vec<u8>,
}
