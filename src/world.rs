use std::vec;

use anyhow::{Ok, Result};
use borsh::BorshDeserialize;
use solana_signer::Signer;

use crate::{
    account::WorldAccount,
    client::WorldClient,
    instructions::{create_world_ix, write_to_world_ix},
    pda::find_world_pda
};

pub struct World;

impl World {
    pub fn create(client: &WorldClient, payer: &impl Signer, name: &str) -> Result<()> {
        let (world_pda, _) = find_world_pda(&payer.pubkey(), name);
        let ix = create_world_ix(payer.pubkey(), world_pda, name.to_string());

        client.send_ixs(payer, vec![ix])?;

        Ok(())
    }

    pub fn write(
        client: &WorldClient,
        payer: &impl Signer,
        name: &str,
        data: Vec<u8>,
    ) -> Result<()> {
        let (world_pda, _) = find_world_pda(&payer.pubkey(), name);

        let ix = write_to_world_ix(payer.pubkey(), world_pda, data);

        client.send_ixs(payer, vec![ix])?;
        Ok(())
    }

    pub fn read(client: &WorldClient, payer: &impl Signer, name: &str) -> Result<WorldAccount> {
        let (world_pda, _) = find_world_pda(&payer.pubkey(), name);

        let data = client.rpc.get_account_data(&world_pda)?;

        let world = WorldAccount::try_from_slice(&data)?;

        Ok(world)
    }
}
