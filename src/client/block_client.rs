use std::collections::HashMap;
use std::fmt::format;
use snarkvm::prelude::{Block, Testnet3};
use snarkvm_console_network::Network;
use crate::{AbstractClient};

pub struct BlockClient {
    base_url:String,
}

impl AbstractClient for BlockClient {
    fn new(base_url:String) -> Self {
        BlockClient {
            base_url
        }
    }

    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }
}

impl BlockClient {

    pub async fn query_by_height(&self,height:u32) -> Block<Testnet3>{
        reqwest::get(format!("{}/block/{}",self.get_base_url(),height)).await.unwrap()
        .json::<Block<Testnet3>>()
        .await.unwrap()
    }

    pub async fn query_by_hash(&self,hash:String) -> Block<Testnet3>{
        reqwest::get(format!("{}/block/{}",self.get_base_url(),hash)).await.unwrap()
            .json::<Block<Testnet3>>()
            .await.unwrap()
    }

    pub async fn query_by_range(&self,start_height:u32, end_height:u32) -> Vec<Block<Testnet3>>{
        reqwest::get(format!("{}/blocks?start={}&end={}",self.get_base_url(),start_height,end_height)).await.unwrap()
            .json::<Vec<Block<Testnet3>>>()
            .await.unwrap()
    }

    pub async fn query_height_by_hash(&self,block_hash:String) -> Option<u32>{
        reqwest::get(format!("{}/height/{}",self.get_base_url(),block_hash)).await.unwrap()
            .json::<Option<u32>>()
            .await.unwrap()
    }

    pub async fn query_block_hash_contain_transaction_id(&self, transaction_id:String) -> <Testnet3 as Network>::BlockHash{
        reqwest::get(format!("{}/find/blockHash/{}",self.get_base_url(),transaction_id)).await.unwrap()
            .json::<<Testnet3 as Network>::BlockHash>()
            .await.unwrap()
    }
}