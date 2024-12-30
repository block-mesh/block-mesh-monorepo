use anchor_lang::prelude::*;

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
    /// Number of leaves.
    pub leaves_len: u64,
}

impl MerkleDistributor {
    pub const LEN: usize = 500;
}

#[cfg(test)]
mod tests {
    use crate::state::off_chain::*;
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
    use anchor_lang::AnchorSerialize;
    use rs_merkle::algorithms::Sha256;
    use rs_merkle::{Hasher, MerkleProof, MerkleTree};
    use solana_sdk::signature::{Keypair, Signer};
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};

    pub fn read_keys_from_dir() -> Vec<Keypair> {
        let mut keys: Vec<Keypair> = Vec::new();
        let path = "./test-keys/";
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let mut file = File::open(path).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                let keypair_bytes: Vec<u8> = serde_json::from_str(&content).unwrap();
                let keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
                keys.push(keypair);
            }
        }
        keys
    }

    pub fn write_merkle_to_file(merkle_output: MerkleOutput) {
        let file_path = "./test-merkle/merkle.json";
        let mut file = File::create(file_path).unwrap();
        let merkle_str = serde_json::to_string_pretty(&merkle_output).unwrap();
        file.write_all(merkle_str.as_bytes()).unwrap();
    }

    pub fn read_merkle_from_file() -> MerkleOutput {
        let file_path = "./test-merkle/merkle.json";
        let file_content = fs::read_to_string(file_path).expect("Failed to read the file");
        let merkle: MerkleOutput =
            serde_json::from_str(&file_content).expect("Failed to deserialize JSON");
        merkle
    }

    #[test]
    pub fn rs_merkle() {
        let leaf_values = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let leaves: Vec<[u8; 32]> = leaf_values
            .iter()
            .map(|x| Sha256::hash(x.as_bytes()))
            .collect();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
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
        let keys = read_keys_from_dir();
        let mut leaf_values: Vec<Claimant> = Vec::new();
        for (index, key) in keys.iter().enumerate() {
            leaf_values.push(Claimant {
                claimant: key.pubkey(),
                amount: index as u64 * LAMPORTS_PER_SOL,
            })
        }
        let leaves: Vec<[u8; 32]> = leaf_values
            .iter()
            .map(|x| Sha256::hash(&*x.as_bytes()))
            .collect();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
        let mut merkle_output = MerkleOutput {
            root: Vec::new(),
            leafs: Vec::new(),
        };

        let merkle_root = merkle_tree
            .root()
            .ok_or("couldn't get the merkle root")
            .unwrap();

        merkle_output.root = merkle_root.try_to_vec().unwrap();

        // let indices_to_prove = vec![3, 4];
        for index in 0..leaf_values.len() {
            let indices_to_prove = vec![index];
            let leaves_to_prove = leaves
                .get(index..index + 1)
                .ok_or("can't get leaves to prove")
                .unwrap();
            let merkle_proof = merkle_tree.proof(&indices_to_prove);
            let proof_bytes = merkle_proof.to_bytes();
            // println!("index = {} | proof = {:?}", index, proof_bytes);
            // Parse proof back on the client
            let proof = MerkleProof::<Sha256>::try_from(proof_bytes.clone()).unwrap();
            assert!(proof.verify(
                merkle_root,
                &indices_to_prove,
                leaves_to_prove,
                leaves.len()
            ));
            let leaves_to_prove = leaves_to_prove.iter().map(|i| i.to_vec()).collect();
            merkle_output.leafs.push(Leaf {
                index: index as u64,
                proof: proof_bytes.to_vec(),
                claimant: leaf_values.get(index).cloned().unwrap(),
                leaves_to_prove,
            });
        }

        // println!("merkle_output = {:?}", merkle_output);
        write_merkle_to_file(merkle_output);
    }

    #[test]
    pub fn claim_test_from_file() {
        let merkle_output = read_merkle_from_file();
        let leaf_values = merkle_output.leaf_values();
        let leaves = merkle_output.leaves();
        let merkle_root = merkle_output.root;
        let merkle_root = <[u8; 32]>::try_from(merkle_root).unwrap();
        for index in 0..leaf_values.len() {
            let indices_to_prove = vec![index];
            let leaves_to_prove = leaves
                .get(index..index + 1)
                .ok_or("can't get leaves to prove")
                .unwrap();
            let proof_bytes = merkle_output.leafs.get(index).unwrap().clone().proof;
            let proof = MerkleProof::<Sha256>::try_from(proof_bytes.clone()).unwrap();
            assert!(proof.verify(
                merkle_root,
                &indices_to_prove,
                leaves_to_prove,
                leaves.len()
            ));
        }
    }
}
