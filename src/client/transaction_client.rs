use snarkvm::prelude::{Testnet3, Transaction, Transactions};
use snarkvm_console_network::Network;
use crate::AbstractClient;

pub struct TransactionClient {
    base_url:String,
}

impl AbstractClient for TransactionClient {
    fn new(base_url:String) -> Self {
        TransactionClient {
            base_url
        }
    }

    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }
}

impl TransactionClient {

    pub async fn query_transaction_id_by_transition_id(&self,transition_id:String) -> Option<<Testnet3 as Network>::TransactionID>{
        reqwest::get(format!("{}/find/transactionID/{}",self.get_base_url(),transition_id)).await.unwrap()
            .json::<Option<<Testnet3 as Network>::TransactionID>>()
            .await.unwrap()
    }

    pub async fn query_transaction_id_by_program_id(&self,program_id:String) -> Option<<Testnet3 as Network>::TransactionID>{
        reqwest::get(format!("{}/find/transactionID/{}",self.get_base_url(),program_id)).await.unwrap()
            .json::<Option<<Testnet3 as Network>::TransactionID>>()
            .await.unwrap()
    }

    pub async fn query_transactions_by_height(&self,height:u32) -> Transactions<Testnet3>{
        reqwest::get(format!("{}/block/{}/transactions",self.get_base_url(),height)).await.unwrap()
            .json::<Transactions<Testnet3>>()
            .await.unwrap()
    }

    pub async fn query_transaction_by_id(&self,transaction_id:String) -> Transaction<Testnet3>{
        reqwest::get(format!("{}/transaction/{}",self.get_base_url(),transaction_id)).await.unwrap()
            .json::<Transaction<Testnet3>>()
            .await.unwrap()
    }

    pub async fn query_transactions_of_memory_pool(&self) -> Option<<Testnet3 as Network>::TransactionID>{
        reqwest::get(format!("{}/memoryPool/transactions",self.get_base_url())).await.unwrap()
            .json::<Option<<Testnet3 as Network>::TransactionID>>()
            .await.unwrap()
    }

    // pub async fn broadcast(&self,transition:Transaction<Testnet3>) -> <Testnet3 as Network>::TransactionID {
    //     let client = reqwest::Client::new();
    //     client.post(format!("{}/transaction/broadcast",self.get_base_url()))
    //         .body(transition)
    //         .send()
    //         .await.unwrap()
    //         .json::<<Testnet3 as Network>::TransactionID>( )
    //         .await.unwrap()
    // }




}