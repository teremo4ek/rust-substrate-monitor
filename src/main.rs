use futures::StreamExt;
use sp_keyring::AccountKeyring;
use std::time::Duration;
use subxt::{
    tx::PairSigner,
    OnlineClient,
    PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_path = "./metadata/metadata.scale")]
pub mod polkadot {}

/// Subscribe to all events, and then manually look through them and
/// pluck out the events that we care about.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    // Subscribe to any events that occur:
    let mut event_sub = api.events().subscribe().await?;

    // Our subscription will see the events emitted as a result of this:
    while let Some(events) = event_sub.next().await {
        let events = events?;
        let block_hash = events.block_hash();

        // We can dynamically decode events:
        for event in events.iter() {
            let event = event?;
            let is_balance_transfer = event
                .as_event::<polkadot::balances::events::Transfer>()?
                .is_some();
            let pallet = event.pallet_name();
            let variant = event.variant_name();
            if is_balance_transfer {
                println!("  Dynamic event details: {block_hash:?}:");
                println!("    {pallet}::{variant} (is balance transfer? {is_balance_transfer})");
            }
        }

        // Or we can find the first transfer event, ignoring any others:
        let transfer_event =
            events.find_first::<polkadot::balances::events::Transfer>()?;

        if let Some(ev) = transfer_event {
            println!("  - Balance transfer success: from {:?} - to {:?} - value: {:?}", ev.from, ev.to, ev.amount);
        } 
    }

    Ok(())
}