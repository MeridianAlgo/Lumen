use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Per-account state stored in the global state tree.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AccountState {
    pub nonce: u64,
    pub lusd_balance: u64,
    pub ljun_balance: u64,
    pub lumina_balance: u64,
    pub commitment: Option<[u8; 32]>,
    /// Passkey device key (65 bytes WebAuthn compressed public key)
    pub passkey_device_key: Option<Vec<u8>>,
    /// Social recovery guardians (list of pubkeys)
    pub guardians: Vec<[u8; 32]>,
    /// Post-quantum public key (Dilithium/Falcon), if account has opted in
    pub pq_pubkey: Option<Vec<u8>>,
    /// Cumulative transaction volume for velocity reward calculation (per epoch)
    pub epoch_tx_volume: u64,
    /// Last epoch in which velocity rewards were claimed
    pub last_reward_epoch: u64,
    /// On-chain credit score (0 = unscored, 300..850 mapped to u16)
    pub credit_score: u16,
    /// Active stream payments originated by this account
    pub active_streams: Vec<StreamState>,
    /// Yield token positions
    pub yield_positions: Vec<YieldPosition>,
}

/// Streaming payment state
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamState {
    pub recipient: [u8; 32],
    pub amount_per_sec: u64,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub withdrawn: u64,
}

/// Yield-bearing wrapped token position
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YieldPosition {
    pub token_id: u64,
    pub principal: u64,
    pub maturity_height: u64,
    pub issued_height: u64,
}

/// Global chain state — the complete state of LuminaChain at any height.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GlobalState {
    pub accounts: HashMap<[u8; 32], AccountState>,
    pub total_lusd_supply: u64,
    pub total_ljun_supply: u64,

    // Stability & Tranches
    pub stabilization_pool_balance: u64,
    pub reserve_ratio: f64,
    pub oracle_prices: HashMap<String, u64>,
    pub validators: Vec<ValidatorState>,

    // Protection
    pub circuit_breaker_active: bool,
    pub fair_redeem_queue: Vec<RedemptionRequest>,
    pub last_rebalance_height: u64,

    // Insurance fund
    pub insurance_fund_balance: u64,

    // Custodian marketplace
    pub custodians: Vec<CustodianState>,
    pub last_reserve_rotation_height: u64,

    // Compliance circuits registry
    pub compliance_circuits: HashMap<u64, Vec<u8>>,

    // RWA registry
    pub rwa_listings: HashMap<[u8; 32], RWAListing>,

    // Yield token counter
    pub next_yield_token_id: u64,

    // Health index (0..10000 representing 0.00..100.00)
    pub health_index: u64,

    // Flash mint tracking (per-block, reset each block)
    pub pending_flash_mints: u64,

    // Epoch tracking for velocity rewards
    pub current_epoch: u64,
    pub velocity_reward_pool: u64,
}

impl GlobalState {
    pub fn root_hash(&self) -> [u8; 32] {
        let bytes = bincode::serialize(self).expect("state serialization");
        *blake3::hash(&bytes).as_bytes()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedemptionRequest {
    pub address: [u8; 32],
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidatorState {
    pub pubkey: [u8; 32],
    pub stake: u64,
    pub power: u64,
    pub is_green: bool,
    pub energy_proof: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustodianState {
    pub pubkey: [u8; 32],
    pub stake: u64,
    pub mpc_pubkeys: Vec<[u8; 32]>,
    pub registered_height: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RWAListing {
    pub owner: [u8; 32],
    pub attestation_proof: Vec<u8>,
    pub collateral_value: u64,
    pub is_active: bool,
    pub collateralized_amount: u64,
}
