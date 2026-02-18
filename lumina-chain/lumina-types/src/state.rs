use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AccountState {
    pub nonce: u64,
    pub lusd_balance: u64,
    pub ljun_balance: u64,
    pub lumina_balance: u64, // Native gas token
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GlobalState {
    pub accounts: HashMap<[u8; 32], AccountState>,
    pub total_lusd_supply: u64,
    pub total_ljun_supply: u64,
    pub reserve_ratio: f64, // Senior tranche backing ratio
    pub oracle_prices: HashMap<String, u64>,
    pub validators: Vec<ValidatorState>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidatorState {
    pub pubkey: [u8; 32],
    pub stake: u64,
    pub power: u64,
}
