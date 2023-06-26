use snarkvm::prelude::Testnet3;
use snarkvm_console_network::Network;
use crate::AbstractClient;

pub struct TransitionClient {
    base_url:String,
}

impl AbstractClient for TransitionClient {
    fn new(base_url:String) -> Self {
        TransitionClient {
            base_url
        }
    }

    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }
}

impl TransitionClient {

    pub async fn query_transition_id_by_input_id(&self,input_id:String) -> Option<<Testnet3 as Network>::TransitionID>{
        reqwest::get(format!("{}/find/transactionID/{}",self.get_base_url(),input_id)).await.unwrap()
            .json::<Option<<Testnet3 as Network>::TransitionID>>()
            .await.unwrap()
    }

    pub async fn query_transition_id_by_output_id(&self,output_id:String) -> Option<<Testnet3 as Network>::TransitionID>{
        reqwest::get(format!("{}/find/transactionID/{}",self.get_base_url(),output_id)).await.unwrap()
            .json::<Option<<Testnet3 as Network>::TransitionID>>()
            .await.unwrap()
    }
}