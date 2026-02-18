use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StablecoinInstruction {
    // === Core Asset Operations ===
    RegisterAsset { ticker: String, decimals: u8 },
    MintSenior { amount: u64, collateral_amount: u64, proof: Vec<u8> },
    RedeemSenior { amount: u64 },
    MintJunior { amount: u64, collateral_amount: u64 },
    RedeemJunior { amount: u64 },
    Burn { amount: u64, asset: AssetType },
    Transfer { to: [u8; 32], amount: u64, asset: AssetType },

    // === Stability & Tranche Management ===
    RebalanceTranches,
    DistributeYield { total_yield: u64 },
    TriggerStabilizer,
    RunCircuitBreaker { active: bool },
    FairRedeemQueue { batch_size: u32 },

    // === Privacy & Compliance ===
    ConfidentialTransfer { commitment: [u8; 32], proof: Vec<u8> },
    ProveCompliance { tx_hash: [u8; 32], proof: Vec<u8> },
    ZkTaxAttest { period: u64, proof: Vec<u8> },
    MultiJurisdictionalCheck { jurisdiction_id: u32, proof: Vec<u8> },

    // === Oracle & Reserves ===
    UpdateOracle { asset: String, price: u64, timestamp: u64, signature: Vec<u8> },
    SubmitZkPoR { proof: Vec<u8>, total_reserves: u64, timestamp: u64 },

    // === Advanced DeFi & Fiat Hooks ===
    InstantFiatBridge { amount: u64, target_bank_id: [u8; 16], mpc_sig: Vec<u8> },
    ZeroSlipBatchMatch { orders: Vec<[u8; 32]> },
    DynamicHedge { ratio: f64 },
    GeoRebalance { zone_id: u32 },
    VelocityIncentive { multiplier: f32 },
    StreamPayment { to: [u8; 32], amount_per_sec: u64, duration: u64 },

    // === Governance & Staking ===
    RegisterValidator { pubkey: [u8; 32], stake: u64 },
    Vote { proposal_id: u64, vote: bool },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AssetType {
    LUSD,
    LJUN,
    Lumina (u64), // Native gas token
    Custom(String),
}
