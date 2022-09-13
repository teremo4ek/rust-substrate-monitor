use subxt::{
    OnlineClient,
    PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_path = "./metadata/metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let address = polkadot::storage().system().account_root();

    let mut iter = api.storage().iter(address, 10, None).await?;

    while let Some((key, account)) = iter.next().await? {
        println!("{}: {}", hex::encode(key), account.data.free);
    }
    Ok(())
}