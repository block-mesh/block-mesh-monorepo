use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

use rs_merkle::algorithms::Sha256;
use rs_merkle::Hasher;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Claimant {
    pub claimant: Pubkey,
    pub amount: u64,
}

impl Claimant {
    #[cfg(test)]
    pub fn new() -> Self {
        use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
        use rand::Rng;
        use solana_sdk::signature::{Keypair, Signer};
        let key = Keypair::new();
        let mut rng = rand::thread_rng();
        let amount = rng.gen_range(0..LAMPORTS_PER_SOL);
        Self {
            claimant: key.pubkey(),
            amount,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::with_capacity(100);
        vec.extend(&self.claimant.to_bytes());
        vec.extend(&self.amount.to_le_bytes());
        vec
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Leaf {
    pub index: u64,
    pub proof: Vec<u8>,
    pub claimant: Claimant,
    pub leaves_to_prove: Vec<Vec<u8>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct MerkleOutput {
    pub root: Vec<u8>,
    pub leafs: Vec<Leaf>,
}

impl MerkleOutput {
    pub fn leaf_values(&self) -> Vec<Claimant> {
        let vec: Vec<Claimant> = self.leafs.iter().map(|i| i.claimant.clone()).collect();
        vec
    }

    pub fn leaves(&self) -> Vec<[u8; 32]> {
        let leaf_values = self.leaf_values();
        let leaves: Vec<[u8; 32]> = leaf_values
            .iter()
            .map(|x| Sha256::hash(&x.as_bytes()))
            .collect();
        leaves
    }
}
