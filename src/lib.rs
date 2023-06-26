mod client;
pub use client::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use snarkvm::prelude::ConfirmedTransaction;
    use snarkvm::prelude::Instruction::AssertEq;
    use snarkvm_console_network::Testnet3;
    use super::*;

    #[tokio::test]
    async fn test_block() {

        let task = tokio::task::spawn(async {
            let block_client = BlockClient::new("https://vm.aleo.org/api/testnet3".to_string());
            let result1 = block_client.query_by_height(1).await;

            let result2 = block_client.query_by_hash(result1.hash().to_string()).await;

            let result3 = block_client.query_by_range(0,2).await;

            let result4 = block_client.query_height_by_hash(result3.get(0).unwrap().hash().to_string()).await;

            let result5 = block_client.query_block_hash_contain_transaction_id(result2.transaction_ids().next().unwrap().to_string()).await;
            (result1.height(),result2.height(),result3.len(),result4,result5,result2.hash())
        });

        let (result1,result2,result3,result4,result5,result6) = task.await.expect("fail to test");
        assert_eq!(result1,1);
        assert_eq!(result2,1);
        assert_eq!(result3,2);
        assert_eq!(result4.unwrap(),0);
        assert_eq!(result5,result6);
    }

    #[tokio::test]
    async fn test_node() {
    }


}
