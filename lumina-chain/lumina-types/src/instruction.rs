use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TrancheType {
    Senior,
    Junior,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StablecoinInstruction {
    // Core
    RegisterAsset { symbol: String, is_senior: bool },
    MintLUM { amount: u64, tranche: TrancheType },
    BurnLUM { amount: u64 },
    Transfer { to: [u8; 32], amount: u64 },

    // Tranche Management
    MintSenior { amount: u64 },
    MintJunior { amount: u64 },
    RebalanceTranches,

    // Stability & Yield
    SubmitZkPor { proof: Vec<u8> },
    DistributeYield,
    TriggerStabilizer,
    RunCircuitBreaker,
    FairRedeemQueue { request_id: u64 },

    // Privacy & Compliance
    ConfidentialTransfer { encrypted_data: Vec<u8>, proof: Vec<u8> },
    ProveCompliance { proof: Vec<u8> },
    ZkTaxAttest { tax_proof: Vec<u8> },
    MultiJurisdictionalCheck { region: String },

    // Fiat & Interop
    InstantFiatBridge { bank_proof: Vec<u8> },
    ZeroSlipBatchMatch { batch_id: u64 },
    DynamicHedge { strategy: u8 },
    GeoRebalance { region: String, amount: u64 },
    VelocityIncentive { velocity_score: u64 },
    StreamPayment { stream_id: u64, rate: u64 },
}
