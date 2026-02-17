use clap::Parser;
use anyhow::Result;
use tokio::signal;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    validator: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Starting Lumina Node...");

    // Init Storage
    let storage = lumina_storage::db::Storage::new("./data").expect("Failed to initialize storage");
    println!("Storage initialized at ./data");

    // Init Network
    tokio::spawn(async {
        if let Err(e) = lumina_network::start_p2p().await {
            eprintln!("P2P Network failed: {}", e);
        }
    });

    // Init Consensus
    tokio::spawn(async {
        lumina_consensus::start_consensus().await;
    });

    // Init API
    tokio::spawn(async {
        lumina_api::start_server().await;
    });

    println!("Node running. Press Ctrl+C to stop.");
    signal::ctrl_c().await?;
    
    Ok(())
}
