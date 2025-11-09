// temporary file | they should be pulled from onchain crate

use bytemuck::{Pod, Zeroable};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MojoInstructions {
    CreateAccount = 0,
    DelegateAccount = 1,
    Commit = 2,
    UpdateDelegatedAccount = 3,
    UndelegateAccount = 4,
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq)]
pub struct GenIxHandler {
    pub seeds: [u8; 32],
    pub size: [u8; 8],
}

impl GenIxHandler {
    pub const LEN: usize = core::mem::size_of::<GenIxHandler>();

    pub fn new(size: [u8; 8]) -> Self {
        Self {
            seeds: [0u8; 32],
            size,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bytemuck::bytes_of(self).to_vec()
    }
}
