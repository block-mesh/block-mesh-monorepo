use serde::{Deserialize, Serialize};

use crate::general::jsonrpc::Jsonrpc;
use crate::general::methods::Methods;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendTransactionInput {
    id: u64,
    jsonrpc: Jsonrpc,
    method: Methods,
    params: [String; 1],
}

impl SendTransactionInput {
    pub fn new(transaction: &str) -> Self {
        SendTransactionInput {
            id: 1,
            jsonrpc: Jsonrpc::Jsonrpc,
            method: Methods::SendTransaction,
            params: [transaction.to_string()],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendTransactionOutput {
    id: u64,
    jsonrpc: Jsonrpc,
    pub result: String,
}

#[cfg(test)]
mod tests {
    use solana_sdk::message::Message;
    use solana_sdk::signature::Keypair;
    use solana_sdk::signer::Signer;
    use solana_sdk::transaction::Transaction;
    use spl_memo::build_memo;

    use crate::client::rpc_client::RpcClient;
    use crate::general::commitment::Commitment;
    use crate::{PAYER_KEYPAIR, PUBLIC_URLS};

    #[tokio::test]
    async fn test_send_transaction() {
        let client = RpcClient::new(PUBLIC_URLS[2].to_string(), Commitment::Confirmed);
        let memo = "Hello World";
        let payer_keypair = Keypair::from_bytes(&PAYER_KEYPAIR).unwrap();
        // let signer_keypair = Keypair::from_bytes(&SIGNER_KEYPAIR).unwrap();
        let latest_blockhash = match client.get_latest_blockhash(None, None).await {
            Err(_) => return,
            Ok(r) => r.result.value.blockhash,
        };
        let instruction = build_memo(memo.as_bytes(), &[]);
        let mut transaction =
            Transaction::new_unsigned(Message::new(&[instruction], Some(&payer_keypair.pubkey())));

        transaction
            .try_partial_sign(&vec![&payer_keypair], latest_blockhash.parse().unwrap())
            .unwrap();
        let serialized_txn = RpcClient::serialize_and_encode(&transaction).unwrap();

        // let echo_json: serde_json::Value = reqwest::Client::new()
        //     .post(PUBLIC_URLS[2])
        //     .json(&SendTransactionInput::new(&serialized_txn))
        //     .send()
        //     .await
        //     .unwrap()
        //     .json()
        //     .await
        //     .unwrap();
        //
        // println!("{:?}", echo_json);
        client
            .send_transaction(None, &serialized_txn)
            .await
            .unwrap();
    }
}
