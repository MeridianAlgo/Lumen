use axum::{
    routing::{get, post},
    Router, Json, extract::{State, Path},
};
use lumina_types::transaction::Transaction;
use lumina_types::block::Block;
use lumina_types::state::GlobalState;
use lumina_storage::db::Storage;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub global_state: Arc<RwLock<GlobalState>>,
    pub storage: Arc<Storage>,
    pub tx_sender: mpsc::Sender<Transaction>,
}

pub async fn start_server(
    global_state: Arc<RwLock<GlobalState>>,
    storage: Arc<Storage>,
    tx_sender: mpsc::Sender<Transaction>,
) {
    let state = AppState { global_state, storage, tx_sender };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/state", get(get_state))
        .route("/health", get(get_health))
        .route("/tx/signing_bytes", post(tx_signing_bytes))
        .route("/tx", post(submit_tx))
        .route("/block/{height}", get(get_block))
        .route("/account/{address}", get(get_account))
        .route("/faucet", post(faucet))
        .route("/validators", get(get_validators))
        .route("/insurance", get(get_insurance))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "LuminaChain API v1.0 — Production L1 Stablecoin Network"
}

async fn get_state(State(state): State<AppState>) -> Json<serde_json::Value> {
    let guard = state.global_state.read().await;
    let summary = serde_json::json!({
        "total_lusd_supply": guard.total_lusd_supply,
        "total_ljun_supply": guard.total_ljun_supply,
        "reserve_ratio": guard.reserve_ratio,
        "stabilization_pool_balance": guard.stabilization_pool_balance,
        "circuit_breaker_active": guard.circuit_breaker_active,
        "insurance_fund_balance": guard.insurance_fund_balance,
        "health_index": guard.health_index,
        "validator_count": guard.validators.len(),
        "custodian_count": guard.custodians.len(),
        "rwa_listing_count": guard.rwa_listings.len(),
        "pending_redeem_queue": guard.fair_redeem_queue.len(),
        "current_epoch": guard.current_epoch,
        "velocity_reward_pool": guard.velocity_reward_pool,
        "account_count": guard.accounts.len(),
    });
    Json(summary)
}

async fn get_health(State(state): State<AppState>) -> Json<serde_json::Value> {
    let guard = state.global_state.read().await;
    let health = serde_json::json!({
        "health_index": guard.health_index,
        "health_pct": format!("{:.2}%", guard.health_index as f64 / 100.0),
        "reserve_ratio": guard.reserve_ratio,
        "circuit_breaker_active": guard.circuit_breaker_active,
        "insurance_fund_balance": guard.insurance_fund_balance,
        "green_validator_count": guard.validators.iter().filter(|v| v.is_green).count(),
        "total_validator_count": guard.validators.len(),
    });
    Json(health)
}

async fn get_block(
    State(state): State<AppState>,
    Path(height): Path<u64>,
) -> Json<Option<Block>> {
    match state.storage.load_block_by_height(height) {
        Ok(block) => Json(block),
        Err(_) => Json(None),
    }
}

#[derive(serde::Deserialize)]
struct UnsignedTxRequest {
    pub sender: [u8; 32],
    pub nonce: u64,
    pub instruction: lumina_types::instruction::StablecoinInstruction,
    pub gas_limit: u64,
    pub gas_price: u64,
}

async fn tx_signing_bytes(Json(req): Json<UnsignedTxRequest>) -> Json<serde_json::Value> {
    let tx = Transaction {
        sender: req.sender,
        nonce: req.nonce,
        instruction: req.instruction,
        signature: Vec::new(),
        gas_limit: req.gas_limit,
        gas_price: req.gas_price,
    };

    let signing_bytes = tx.signing_bytes();
    Json(serde_json::json!({
        "signing_bytes_hex": hex::encode(signing_bytes),
    }))
}

async fn get_account(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Json<serde_json::Value> {
    let guard = state.global_state.read().await;
    let addr_hex = address.trim_start_matches("0x");
    if let Ok(bytes) = hex::decode(addr_hex) {
        if bytes.len() == 32 {
            let mut key = [0u8; 32];
            key.copy_from_slice(&bytes);
            if let Some(account) = guard.accounts.get(&key) {
                return Json(serde_json::json!({
                    "address": address,
                    "lusd_balance": account.lusd_balance,
                    "ljun_balance": account.ljun_balance,
                    "lumina_balance": account.lumina_balance,
                    "nonce": account.nonce,
                    "has_passkey": account.passkey_device_key.is_some(),
                    "guardian_count": account.guardians.len(),
                    "has_pq": account.pq_pubkey.is_some(),
                    "credit_score": account.credit_score,
                    "yield_positions": account.yield_positions.len(),
                    "active_streams": account.active_streams.len(),
                }));
            }
        }
    }
    Json(serde_json::json!({"error": "Account not found"}))
}

async fn submit_tx(
    State(state): State<AppState>,
    Json(tx): Json<Transaction>,
) -> Json<serde_json::Value> {
    let tx_id = hex::encode(tx.id());
    match state.tx_sender.send(tx).await {
        Ok(_) => Json(serde_json::json!({
            "status": "submitted",
            "tx_id": tx_id,
        })),
        Err(_) => Json(serde_json::json!({
            "status": "failed",
            "error": "Channel full or closed",
        })),
    }
}

async fn faucet(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let amount: u64 = 10_000;

    let addr_hex = req
        .get("address")
        .and_then(|v| v.as_str())
        .unwrap_or("0x")
        .trim()
        .trim_start_matches("0x");

    let Ok(bytes) = hex::decode(addr_hex) else {
        return Json(serde_json::json!({
            "status": "failed",
            "error": "invalid address hex"
        }));
    };
    if bytes.len() != 32 {
        return Json(serde_json::json!({
            "status": "failed",
            "error": "address must be 32 bytes"
        }));
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&bytes);

    let mut guard = state.global_state.write().await;
    let account = guard.accounts.entry(key).or_default();
    account.lusd_balance = account.lusd_balance.saturating_add(amount);
    guard.total_lusd_supply = guard.total_lusd_supply.saturating_add(amount);

    Json(serde_json::json!({
        "status": "funded",
        "address": format!("0x{}", addr_hex),
        "amount": amount,
        "asset": "LUSD"
    }))
}

async fn get_validators(State(state): State<AppState>) -> Json<serde_json::Value> {
    let guard = state.global_state.read().await;
    let validators: Vec<serde_json::Value> = guard
        .validators
        .iter()
        .map(|v| {
            serde_json::json!({
                "pubkey": hex::encode(v.pubkey),
                "stake": v.stake,
                "power": v.power,
                "is_green": v.is_green,
            })
        })
        .collect();
    Json(serde_json::json!({ "validators": validators }))
}

async fn get_insurance(State(state): State<AppState>) -> Json<serde_json::Value> {
    let guard = state.global_state.read().await;
    Json(serde_json::json!({
        "insurance_fund_balance": guard.insurance_fund_balance,
        "total_lusd_supply": guard.total_lusd_supply,
        "coverage_ratio": if guard.total_lusd_supply > 0 {
            guard.insurance_fund_balance as f64 / guard.total_lusd_supply as f64
        } else {
            1.0
        },
    }))
}
