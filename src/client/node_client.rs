use std::fmt::format;
use std::net::SocketAddr;
use indexmap::IndexSet;
use snarkvm::prelude::{Address, Block, StatePath, Testnet3};
use snarkos_node_env::EnvInfo;
use snarkos_node_messages::NodeType;
use snarkvm_console_network::Network;
use crate::AbstractClient;

pub struct NodeClient {
    base_url:String,
}

impl AbstractClient for NodeClient {
    fn new(base_url:String) -> Self {
        NodeClient{
            base_url
        }
    }

    fn get_base_url(&self) -> String {
        self.base_url.clone()
    }
}

impl NodeClient {

    pub async fn query_latest_height(&self) -> u32{
        reqwest::get(format!("{}/latest/height",self.get_base_url())).await.unwrap()
            .json::<u32>()
            .await.unwrap()
    }

    pub async fn query_latest_hash(&self) -> <Testnet3 as Network>::BlockHash{
        reqwest::get(format!("{}/latest/hash",self.get_base_url())).await.unwrap()
            .json::<<Testnet3 as Network>::BlockHash>()
            .await.unwrap()
    }


    pub async fn query_latest_state_root(&self) -> <Testnet3 as Network>::StateRoot{
        reqwest::get(format!("{}/latest/stateRoot",self.get_base_url())).await.unwrap()
            .json::<<Testnet3 as Network>::StateRoot>()
            .await.unwrap()
    }

    pub async fn query_latest(&self) -> Block<Testnet3>{
        reqwest::get(format!("{}/latest/block",self.get_base_url())).await.unwrap()
            .json::<Block<Testnet3>>()
            .await.unwrap()
    }

    // pub async fn query_node_env(&self) -> EnvInfo{
    //     reqwest::get(format!("{}/node/env",self.get_base_url())).await.unwrap()
    //         .json::<EnvInfo>()
    //         .await.unwrap()
    // }

    pub async fn query_node_address(&self) -> Address<Testnet3>{
        reqwest::get(format!("{}/node/address",self.get_base_url())).await.unwrap()
            .json::<Address<Testnet3>>()
            .await.unwrap()
    }

    // pub async fn query_beacons(&self) -> IndexSet<Address<Testnet3>>{
    //     reqwest::get(format!("{}/beacons",self.get_base_url())).await.unwrap()
    //         .json::<IndexSet<Address<Testnet3>>>()
    //         .await.unwrap()
    // }

    pub async fn query_peers_count(&self) -> usize{
        reqwest::get(format!("{}/peers/count",self.get_base_url())).await.unwrap()
            .json::<usize>()
            .await.unwrap()
    }

    pub async fn query_peers_all(&self) -> Vec<SocketAddr>{
        reqwest::get(format!("{}/peers/all",self.get_base_url())).await.unwrap()
            .json::<Vec<SocketAddr>>()
            .await.unwrap()
    }

    pub async fn query_peers_all_metrics(&self) -> Vec<(SocketAddr, NodeType)>{
        reqwest::get(format!("{}/peers/all/metrics",self.get_base_url())).await.unwrap()
            .json::<Vec<(SocketAddr, NodeType)>>()
            .await.unwrap()
    }

    pub async fn query_state_path_by_commitment(&self,commitment:String) -> StatePath<Testnet3>{
        reqwest::get(format!("{}/statePath/{}",self.get_base_url(),commitment)).await.unwrap()
            .json::<StatePath<Testnet3>>()
            .await.unwrap()
    }
}