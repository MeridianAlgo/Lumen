use clap::{Parser, Subcommand};
use anyhow::Result;
use lumina_types::transaction::Transaction;
use lumina_types::instruction::{StablecoinInstruction, AssetType};
use lumina_crypto::signatures::{generate_keypair, sign};
use reqwest::Client;
use ed25519_dalek::Signer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, default_value = "http://localhost:3000")]
    node_url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new keypair
    Keygen,
    /// Mint stablecoin (Testnet only)
    Mint {
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        asset: String, // senior/junior
    },
    /// Transfer tokens
    Transfer {
        #[arg(long)]
        to: String, // hex
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        asset: String, // lusd/ljun
    },
    /// Get account balance
    Balance {
        #[arg(long)]
        address: String, // hex
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = Client::new();

    match &cli.command {
        Commands::Keygen => {
            let kp = generate_keypair();
            println!("New Keypair Generated:");
            println!("Public Key (hex): {}", hex::encode(kp.verifying_key().as_bytes()));
            println!("Private Key (hex): {}", hex::encode(kp.to_bytes()));
        }
        Commands::Mint { amount, asset } => {
            // For testnet, we assume we are minting to ourselves (random key for now)
            // In reality, we'd load a key from file.
            let kp = generate_keypair(); 
            let sender = kp.verifying_key().to_bytes();

            let instruction = match asset.to_lowercase().as_str() {
                "senior" | "lusd" => StablecoinInstruction::MintSenior {
                    amount: *amount,
                    collateral_amount: 0,
                    proof: vec![],
                },
                "junior" | "ljun" => StablecoinInstruction::MintJunior {
                    amount: *amount,
                    collateral_amount: 0,
                },
                _ => panic!("Invalid asset type"),
            };

            let mut tx = Transaction {
                sender,
                nonce: 0, // In real CLI, fetch nonce from node
                instruction,
                signature: vec![],
                gas_limit: 100000,
                gas_price: 1,
            };

            // Sign
            // let bytes = bincode::serialize(&tx).unwrap(); // This includes signature field which is empty?
            // Usually we sign (sender, nonce, instruction, gas).
            // Simplified signing for now:
            tx.signature = sign(&kp, &bincode::serialize(&tx.instruction).unwrap()); // Hacky signature

            let res = client.post(format!("{}/tx", cli.node_url))
                .json(&tx)
                .send()
                .await?;
            
            println!("Response: {}", res.text().await?);
        }
        Commands::Transfer { to, amount, asset } => {
            // Placeholder logic
            println!("Transfer logic not fully implemented in CLI (needs wallet management)");
        }
        Commands::Balance { address } => {
            let res = client.get(format!("{}/state", cli.node_url))
                .send()
                .await?
                .json::<lumina_types::state::GlobalState>()
                .await?;
            
            // Find account
            // Need to decode address hex to [u8; 32]
            println!("State fetched. Total LUSD: {}", res.total_lusd_supply);
        }
    }

    Ok(())
}
