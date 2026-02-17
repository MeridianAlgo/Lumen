use clap::{Parser, Subcommand};
use anyhow::Result;
use lumina_crypto::signatures::generate_keypair;
use lumina_types::instruction::TrancheType;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Keygen,
    Mint { amount: u64, tranche: String },
    Transfer { to: String, amount: u64 },
    Balance { address: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Keygen => {
            let kp = generate_keypair();
            println!("New Keypair generated");
            // In a real CLI, we would serialize and print the public/private keys
        }
        Commands::Mint { amount, tranche } => {
             println!("Minting {} to {}", amount, tranche);
             let tranche_type = match tranche.as_str() {
                 "senior" => TrancheType::Senior,
                 "junior" => TrancheType::Junior,
                 _ => panic!("Invalid tranche"),
             };
             // Construct MintLUM transaction logic here
        }
        Commands::Transfer { to, amount } => {
            println!("Transferring {} to {}", amount, to);
        }
        Commands::Balance { address } => {
            println!("Checking balance for {}", address);
        }
    }
    
    Ok(())
}
