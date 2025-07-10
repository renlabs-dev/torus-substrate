use futures::StreamExt;
use torus_client::client::TorusClient;

#[tokio::test]
async fn list_stakers() {
    let client = TorusClient::for_mainnet().await.unwrap();
    let mut foo = client.torus0().storage().staking_to_iter().await.unwrap();

    let mut val = 0;
    while let Some(foo) = foo.next().await {
        let ((staker, staked), amount) = foo.unwrap();
        eprintln!("{staker} -> {staked}: {amount}");

        val += amount;
    }

    eprintln!("{val}");
}
