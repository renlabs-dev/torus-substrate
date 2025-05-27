use std::{ops::Deref, sync::Arc, time::Instant, u64};

use codec::Encode;
use futures::executor::block_on;
use keccak_hash::keccak_256;
use rand::Rng;
use sha2::Digest;
use subxt::{
    blocks::Block,
    config::polkadot::U256,
    utils::{AccountId32, MultiAddress, H256},
    OnlineClient, PolkadotConfig, SubstrateConfig,
};
use tokio::{
    sync::{broadcast, oneshot, RwLock},
    task::JoinSet,
};
use torus_runtime::configs::BlockLength;

use crate::{interfaces, TorusClient};

pub async fn perform(client: &TorusClient, address: MultiAddress<AccountId32, ()>) {
    let start = Instant::now();

    let num_processes = 12;
    let update_interval = 500000;

    let (tx, _) = broadcast::channel::<(u32, H256)>(num_processes);

    let block_number = client.latest_block_number().await.unwrap();
    let block_hash = client.latest_block_hash().await.unwrap();

    let mut handles = JoinSet::new();

    let address_bytes = Arc::new(address.clone().encode()[1..].to_vec());
    let network = client.network().clone();
    for i in 0..num_processes {
        let mut rx = tx.subscribe();
        let address_bytes = address_bytes.clone();
        handles.spawn(async move {
            let mut block_number: u32 = block_number;
            let mut block_hash: H256 = block_hash;
            let mut block_key_hash =
                create_block_key_hash(&block_hash.as_bytes(), &address_bytes[..]);
            let nonce_limit = u64::MAX - 1;

            let mut nonce_start = u64::from_le_bytes(rand::random()) % nonce_limit;
            let mut nonce_limit = nonce_start + update_interval;

            let difficulty: U256 = U256::from(1_000_000);

            loop {
                while let Ok(block) = rx.try_recv() {
                    block_number = block.0;
                    println!("worker#{i} received new block: {block_number}");
                    block_hash = block.1;
                    block_key_hash =
                        create_block_key_hash(&block_hash.as_bytes(), &address_bytes[1..])
                }

                for nonce in nonce_start..=nonce_limit {
                    let mut nonce_bk_bytes = [0u8; 40];
                    let (first_half, second_half) = nonce_bk_bytes.split_at_mut(8);
                    first_half.copy_from_slice(&nonce.to_le_bytes());
                    second_half.copy_from_slice(&block_key_hash);

                    let nonce_bk_sha = sha2::Sha256::digest(&nonce_bk_bytes[..]);

                    let mut nonce_bk_hash = [0u8; 32];
                    keccak_256(&nonce_bk_sha[..], &mut nonce_bk_hash);

                    let num_hash = U256::from_big_endian(&nonce_bk_hash[..]);
                    let (_, overflowed) = num_hash.overflowing_mul(difficulty);

                    if !overflowed {
                        return WorkResult {
                            block: block_number,
                            nonce,
                            data: nonce_bk_hash.to_vec(),
                        };
                    }
                }

                nonce_start = u64::from_le_bytes(rand::random()) % nonce_limit;
                nonce_limit = nonce_start + update_interval;
            }
        });
    }

    let blocks_client = TorusClient::for_network(network).await.unwrap();
    let blocks = tokio::spawn(async move {
        while let Some(Ok(block)) = blocks_client
            .client()
            .blocks()
            .subscribe_finalized()
            .await
            .unwrap()
            .next()
            .await
        {
            let _ = tx.send((block.number(), block.hash()));
        }
    });

    let WorkResult { block, nonce, data } = handles.join_next().await.unwrap().unwrap();

    println!("found result took {:?}", Instant::now() - start);
    println!(
        "result block {block} current_block {}",
        client.latest_block_number().await.unwrap()
    );

    handles.abort_all();
    blocks.abort();

    println!("calling");
    let call = interfaces::testnet::api::tx()
        .faucet()
        .faucet(block.into(), nonce, data, address);

    let call = client.client().tx().create_unsigned(&call).unwrap();
    // call.submit().await.unwrap();
    dbg!(call
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap());
}

fn create_block_key_hash(block_hash: &[u8], key: &[u8]) -> [u8; 32] {
    let mut block_key_bytes = [0u8; 64];
    let (first_half, second_half) = block_key_bytes.split_at_mut(32);
    first_half.copy_from_slice(block_hash);
    second_half.copy_from_slice(&key[..32]);

    let mut block_key_hash = [0u8; 32];
    keccak_256(&block_key_bytes[..], &mut block_key_hash);

    block_key_hash
}
struct WorkResult {
    block: u32,
    nonce: u64,
    data: Vec<u8>,
}

#[cfg(test)]
mod test {
    use std::{ops::Deref, sync::Arc, time::Instant, u64};

    use crate::Network;
    use codec::Encode;
    use futures::executor::block_on;
    use keccak_hash::keccak_256;
    use rand::Rng;
    use sha2::Digest;
    use subxt::{
        blocks::Block,
        config::polkadot::U256,
        utils::{AccountId32, MultiAddress, H256},
        OnlineClient, PolkadotConfig, SubstrateConfig,
    };
    use tokio::{
        sync::{broadcast, oneshot, RwLock},
        task::JoinSet,
    };
    use torus_runtime::configs::BlockLength;

    use crate::TorusClient;
    use hex::decode;
    use schnorrkel::MiniSecretKey;

    #[tokio::test]
    async fn test() {
        let client = TorusClient::for_network(Network::Testnet).await.unwrap();

        let seed_bytes = decode("seed-bytes").unwrap();

        let mini_secret = MiniSecretKey::from_bytes(&seed_bytes).unwrap();
        let keypair = mini_secret.expand_to_keypair(schnorrkel::ExpansionMode::Ed25519);

        super::perform(&client, MultiAddress::Id(keypair.public.to_bytes().into())).await;
    }
}
