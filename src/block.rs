use std::fmt::Debug;

use crate::{u128_bytes, u32_bytes, u64_bytes, BlockHash, Hashable};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());

        bytes
    }
}
