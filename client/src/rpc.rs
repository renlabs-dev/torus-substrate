use std::marker::PhantomData;

use subxt::backend::rpc::RpcClient;
use subxt::ext::subxt_rpcs::client::RpcParams;
use subxt::utils::AccountId32;
use subxt::{utils::H256, OnlineClient, PolkadotConfig};

use crate::client::TorusClient;

impl<C> TorusClient<C> {
    pub async fn rpc(&self) -> crate::Result<Rpc<C>> {
        let rpc_client = RpcClient::from_insecure_url("wss://api-30.nodes.torus.network").await?;

        Ok(Rpc {
            client: self.client.clone(),
            rpc_client,
            block: None,
            _pd: PhantomData,
        })
    }
}

pub struct Rpc<C> {
    pub(crate) client: OnlineClient<PolkadotConfig>,
    pub(crate) rpc_client: RpcClient,
    pub(crate) block: Option<H256>,
    pub(crate) _pd: PhantomData<C>,
}

impl<C> Rpc<C> {
    pub async fn namespace_path_creation_cost(
        &self,
        account_id: AccountId32,
        path: impl AsRef<str>,
    ) -> crate::Result<(u128, u128)> {
        let mut rpc_params = RpcParams::new();
        rpc_params.push(account_id)?;

        let path = path.as_ref().as_bytes().to_vec();
        rpc_params.push(path)?;

        let res = self
            .rpc_client
            .request("torus0_namespacePathCreationCost", rpc_params)
            .await?;

        Ok(res)
    }

    // pub async fn pending_extrinsics(&self) -> crate::Result<Vec<ExtrinsicData>> {
    //     loop {
    //         let params = RpcParams::new();
    //         let res: serde_json::Value = self
    //             .rpc_client
    //             .request("author_pendingExtrinsics", params)
    //             .await?;

    //         if let serde_json::Value::Array(array) = res {
    //             if !array.is_empty() {
    //                 dbg!(array);
    //             }
    //         }
    //     }

    //     todo!()
    // }
}

// #[tokio::test]
// async fn test() {
//     let a = AccountId32::from_str("5FRjeaM6SpAZC3t8pj9PfTxYS1hoiYn139TMjSi8WRuoo2C1").unwrap();

//     let client = TorusClient::for_mainnet().await.unwrap();
//     // dbg!(client
//     //     .rpc()
//     //     .await
//     //     .unwrap()
//     //     .namespace_path_creation_cost(a, "test.test1.test2.test3")
//     //     .await
//     //     .unwrap());

//     client
//         .rpc()
//         .await
//         .unwrap()
//         .pending_extrinsics()
//         .await
//         .unwrap();
// }
