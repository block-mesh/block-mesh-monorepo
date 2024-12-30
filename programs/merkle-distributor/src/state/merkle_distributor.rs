use anchor_lang::prelude::*;
use rs_merkle::algorithms::Sha256;
use rs_merkle::Hasher;
use rs_merkle::MerkleProof;
use rs_merkle::MerkleTree;
use serde::{Deserialize, Serialize};

/// State for the account which distributes tokens.
#[account]
#[derive(Default)]
pub struct MerkleDistributor {
    /// Base key used to generate the PDA.
    pub signer: Pubkey,
    /// Bump seed.
    pub bump: u8,
    /// The 256-bit merkle root.
    pub root: [u8; 32],
    /// [Mint] of the token to be distributed.
    pub mint: Pubkey,
    /// [Token Account] of the token to be distributed.
    pub token_account: Pubkey,
    /// Maximum number of tokens that can ever be claimed from this [MerkleDistributor].
    pub max_total_claim: u64,
    /// Maximum number of nodes that can ever be claimed from this [MerkleDistributor].
    pub max_num_nodes: u64,
    /// Total amount of tokens that have been claimed.
    pub total_amount_claimed: u64,
    /// Number of nodes that have been claimed.
    pub num_nodes_claimed: u64,
}

impl MerkleDistributor {
    pub const LEN: usize = 500;
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
    use rand::Rng;
    use solana_sdk::signature::{Keypair, Signer};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Claimant {
        pub claimant: Pubkey,
        pub amount: u64,
    }

    impl Claimant {
        pub fn new() -> Self {
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

    #[test]
    pub fn rs_merkle() {
        let leaf_values = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let leaves: Vec<[u8; 32]> = leaf_values
            .iter()
            .map(|x| Sha256::hash(x.as_bytes()))
            .collect();

        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

        println!("merkle_tree.root {:?}", merkle_tree.root());
        println!("merkle_tree.depth {:?}", merkle_tree.depth());
        // let indices_to_prove = vec![3, 4];
        for index in 0..leaf_values.len() {
            let indices_to_prove = vec![index];
            let leaves_to_prove = leaves
                .get(index..index + 1)
                .ok_or("can't get leaves to prove")
                .unwrap();
            let merkle_proof = merkle_tree.proof(&indices_to_prove);
            let merkle_root = merkle_tree
                .root()
                .ok_or("couldn't get the merkle root")
                .unwrap();
            // Serialize proof to pass it to the client
            let proof_bytes = merkle_proof.to_bytes();
            // Parse proof back on the client
            let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();
            assert!(proof.verify(
                merkle_root,
                &indices_to_prove,
                leaves_to_prove,
                leaves.len()
            ));
        }
    }

    #[test]
    pub fn claim_test() {
        let mut leaf_values: Vec<Claimant> = Vec::new();
        for _ in 0..12 {
            leaf_values.push(Claimant::new())
        }
        println!("leaf_values {:?}", leaf_values);
        let leaves: Vec<[u8; 32]> = leaf_values
            .iter()
            .map(|x| Sha256::hash(&*x.as_bytes()))
            .collect();

        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

        println!("merkle_tree.root {:?}", merkle_tree.root());
        println!("merkle_tree.depth {:?}", merkle_tree.depth());

        let merkle_root = merkle_tree
            .root()
            .ok_or("couldn't get the merkle root")
            .unwrap();

        // let indices_to_prove = vec![3, 4];
        for index in 0..leaf_values.len() {
            let indices_to_prove = vec![index];
            let leaves_to_prove = leaves
                .get(index..index + 1)
                .ok_or("can't get leaves to prove")
                .unwrap();
            let merkle_proof = merkle_tree.proof(&indices_to_prove);
            println!("index = {} | proof = {:?}", index, merkle_proof.to_bytes());
            // Serialize proof to pass it to the client
            let proof_bytes = merkle_proof.to_bytes();
            // Parse proof back on the client
            let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();
            assert!(proof.verify(
                merkle_root,
                &indices_to_prove,
                leaves_to_prove,
                leaves.len()
            ));
        }
    }
}
