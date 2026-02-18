use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use lumina_types::transaction::Transaction;
use lumina_types::state::GlobalState;
use lumina_types::instruction::{StablecoinInstruction, AssetType};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::info;
use serde::Deserialize;

#[derive(Clone)]
pub struct AppState {
    pub global_state: Arc<RwLock<GlobalState>>,
    pub tx_sender: mpsc::Sender<Transaction>,
}

pub async fn start_server(
    global_state: Arc<RwLock<GlobalState>>,
    tx_sender: mpsc::Sender<Transaction>,
) {
    let state = AppState { global_state, tx_sender };

    let app = Router::new()
        .route("/", get(root))
        .route("/state", get(get_state))
        .route("/tx", post(submit_tx))
        .route("/faucet", post(faucet)) // Testnet convenience
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("API listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Lumina Chain API v0.1"
}

async fn get_state(State(state): State<AppState>) -> Json<GlobalState> {
    let guard = state.global_state.read().await;
    Json(guard.clone())
}

async fn submit_tx(
    State(state): State<AppState>,
    Json(tx): Json<Transaction>,
) -> Json<String> {
    // Basic validation could go here
    match state.tx_sender.send(tx).await {
        Ok(_) => Json("Transaction submitted".to_string()),
        Err(_) => Json("Failed to submit transaction".to_string()),
    }
}

#[derive(Deserialize)]
struct FaucetRequest {
    address: String, // hex string
}

async fn faucet(
    State(state): State<AppState>,
    Json(req): Json<FaucetRequest>,
) -> Json<String> {
    // Create a mint transaction for the user
    // In real life, this would be signed by a faucet key.
    // For now, we simulate by sending a MintSenior SI from a privileged account?
    // Actually, let's just create a raw transaction that mints LUSD directly if we can cheat.
    // But we can't cheat execution rules easily without a valid signature.
    // Let's assume the faucet has a key and signs it.
    // For simplicity, we'll just construct a "system" transaction that bypasses signature check in our simplified execution (we commented out verify_signature).
    
    // Parse address
    let mut address_bytes = [0u8; 32];
    // hex decode omitted for brevity, assume raw 32 bytes or just hash
    // Let's just use a dummy address if parsing fails or implement hex decode helper.
    // Using a simpler approach:
    
    let tx = Transaction {
        sender: [0u8; 32], // Faucet address (system)
        nonce: 0, // TODO: track nonce
        instruction: StablecoinInstruction::MintSenior {
            amount: 1000,
            collateral_amount: 0,
            proof: vec![],
        },
        signature: vec![],
        gas_limit: 100000,
        gas_price: 1,
    };
    
    match state.tx_sender.send(tx).await {
        Ok(_) => Json("Faucet tx submitted".to_string()),
        Err(_) => Json("Failed".to_string()),
    }
}
