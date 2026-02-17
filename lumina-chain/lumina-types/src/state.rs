use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AccountState {
    pub balance_senior: u64,
    pub balance_junior: u64,
    pub nonce: u64,
    pub is_frozen: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GlobalState {
    pub accounts: HashMap<[u8; 32], AccountState>,
    pub senior_supply: u64,
    pub junior_supply: u64,
    pub stabilization_pool_balance: u64,
    pub target_senior_ratio: f64, // e.g., 0.90 for 90%
    pub current_collateral_ratio: f64,
    pub last_rebalance_height: u64,
    pub is_circuit_breaker_active: bool,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            senior_supply: 0,
            junior_supply: 0,
            stabilization_pool_balance: 0,
            target_senior_ratio: 0.90,
            current_collateral_ratio: 1.0,
            last_rebalance_height: 0,
            is_circuit_breaker_active: false,
        }
    }
}
