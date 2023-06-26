use indexmap::IndexSet;
use snarkvm::prelude::{Identifier, Program, Testnet3, Value};
use crate::AbstractClient;

pub struct ProgramClient {
    base_url:String,
}

impl AbstractClient for ProgramClient {
    fn new(base_url:String) -> Self {
        ProgramClient {
            base_url
        }
    }

    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }
}

impl ProgramClient {
    pub async fn query_program_by_id(&self,program_id:String) -> Program<Testnet3>{
        reqwest::get(format!("{}/program/{}",self.get_base_url(),program_id)).await.unwrap()
            .json::<Program<Testnet3>>()
            .await.unwrap()
    }

    pub async fn query_mapping_names_by_program_id(&self,program_id:String) -> IndexSet<Identifier<Testnet3>>{
        reqwest::get(format!("{}/program/{}/mappings",self.get_base_url(),program_id)).await.unwrap()
            .json::<IndexSet<Identifier<Testnet3>>>()
            .await.unwrap()
    }

    pub async fn query_mapping_value(&self,program_id:String,mapping_name:String,mapping_key:String) -> Value<Testnet3>{
        reqwest::get(format!("{}/program/{}/mapping/{}/{}",self.get_base_url(),program_id,mapping_name,mapping_key)).await.unwrap()
            .json::<Value<Testnet3>>()
            .await.unwrap()
    }
}