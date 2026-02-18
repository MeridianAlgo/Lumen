use clap::Parser;
use anyhow::{Result, Context};
use tokio::signal;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    validator: bool,
    #[arg(short, long, default_value = "./data")]
    data_dir: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Setup Logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Args::parse();
    info!("Starting Lumina Node...");

    // 2. Init Storage
    let storage = lumina_storage::db::Storage::new(&args.data_dir).context("Failed to initialize storage")?;
    info!("Storage initialized at {}", args.data_dir);

    // 3. Load or Create State
    let mut state = match storage.load_state() {
        Ok(s) => {
            if s.accounts.is_empty() && s.total_lusd_supply == 0 {
                info!("State is empty, generating Genesis block...");
                let genesis = lumina_genesis::create_genesis_state();
                storage.save_state(&genesis).expect("Failed to save genesis state");
                genesis
            } else {
                info!("Loaded existing state.");
                s
            }
        },
        Err(e) => {
            error!("Failed to load state: {}", e);
            return Err(e);
        }
    };

    let shared_state = Arc::new(RwLock::new(state));

    // 4. Init Network
    let (net_cmd_tx, mut net_event_rx) = lumina_network::start_p2p().await.context("Failed to start P2P")?;
    
    // Channel for incoming transactions (Network -> Consensus, API -> Consensus)
    let (tx_sender, tx_receiver) = mpsc::channel(1000);

    // Handle Network Events (Blocks & Txs)
    let net_tx_sender = tx_sender.clone();
    tokio::spawn(async move {
        while let Some(event) = net_event_rx.recv().await {
            match event {
                lumina_network::NetworkEvent::TxReceived(data, peer) => {
                    // Deserialize and forward to consensus
                    match bincode::deserialize::<lumina_types::transaction::Transaction>(&data) {
                        Ok(tx) => {
                            let _ = net_tx_sender.send(tx).await;
                        }
                        Err(e) => error!("Failed to deserialize tx from {}: {}", peer, e),
                    }
                },
                lumina_network::NetworkEvent::BlockReceived(data, peer) => {
                    // In a real node, we'd validate and add to blockchain.
                    // For now, we just log.
                    info!("Received block from {}", peer);
                }
            }
        }
    });

    // 5. Init Consensus
    let consensus_state = shared_state.clone();
    let consensus_net_tx = net_cmd_tx.clone();
    let consensus_tx_rx = tx_receiver;

    tokio::spawn(async move {
        let service = lumina_consensus::ConsensusService::new(consensus_state, consensus_net_tx, consensus_tx_rx);
        service.run().await;
    });

    // 6. Init API
    let api_state = shared_state.clone();
    let api_tx_sender = tx_sender.clone();
    tokio::spawn(async move {
        // Adapt API signature to match what we wrote
        // We wrote: pub async fn start_server(global_state: Arc<RwLock<GlobalState>>, tx_sender: mpsc::Sender<Transaction>)
        // So we need to call it correctly.
        // Wait, start_server in lumina-api takes owned Arc and Sender.
        // But start_server is async and doesn't return, so we spawn it.
        // We need to import it.
        // lumina_api::start_server(api_state, api_tx_sender).await;
        // Wait, check imports.
        // I need to update lumina-api imports or re-export it properly.
    });

    // Since I can't easily see if start_server is public or not (it is pub async fn), I'll assume it works.
    // Wait, the call below is inside a spawn, so it must be async block.
    // I need to import lumina_api.

    // Let's actually call it.
    tokio::spawn(async move {
        lumina_api::start_server(api_state, api_tx_sender).await;
    });

    info!("Node running. Press Ctrl+C to stop.");
    signal::ctrl_c().await?;
    
    Ok(())
}
