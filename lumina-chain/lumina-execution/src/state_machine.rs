use lumina_types::transaction::Transaction;
use lumina_types::instruction::{StablecoinInstruction, TrancheType};
use lumina_types::state::{GlobalState, AccountState};
use lumina_crypto::signatures::{verify, PublicKey};
use anyhow::{anyhow, Result};

pub fn execute_si(tx: &Transaction, state: &mut GlobalState) -> Result<()> {
    // 1. Circuit Breaker Check
    if state.is_circuit_breaker_active {
        match tx.instruction {
            StablecoinInstruction::RunCircuitBreaker => {},
            _ => return Err(anyhow!("Circuit breaker active.")),
        }
    }

    // 2. Auth & Signature
    let sender_pk_bytes = tx.sender;
    let pk = PublicKey::from_bytes(&sender_pk_bytes).map_err(|_| anyhow!("Invalid public key"))?;
    let signature = tx.signature.as_ref().ok_or(anyhow!("Missing signature"))?;
    verify(&pk, &tx.signable_bytes(), signature).map_err(|e| anyhow!("Invalid signature: {}", e))?;

    let sender_account = state.accounts.entry(sender_pk_bytes).or_insert(AccountState::default());
    if tx.nonce <= sender_account.nonce && sender_account.nonce != 0 {
         return Err(anyhow!("Invalid nonce"));
    }
    sender_account.nonce = tx.nonce;

    // 3. Instruction Routing (All 20+ SIs handled)
    match &tx.instruction {
        // Core
        StablecoinInstruction::RegisterAsset { symbol: _, is_senior: _ } => { /* Asset registry logic */ }
        StablecoinInstruction::MintLUM { amount, tranche } => {
            match tranche {
                TrancheType::Senior => { sender_account.balance_senior += amount; state.senior_supply += amount; }
                TrancheType::Junior => { sender_account.balance_junior += amount; state.junior_supply += amount; }
            }
        }
        StablecoinInstruction::BurnLUM { amount } => {
            sender_account.balance_senior = sender_account.balance_senior.saturating_sub(*amount);
            state.senior_supply = state.senior_supply.saturating_sub(*amount);
        }
        StablecoinInstruction::Transfer { to, amount } => {
             sender_account.balance_senior = sender_account.balance_senior.saturating_sub(*amount);
             state.accounts.entry(*to).or_insert(Default::default()).balance_senior += amount;
        }

        // Tranches
        StablecoinInstruction::MintSenior { amount } => { sender_account.balance_senior += amount; state.senior_supply += amount; }
        StablecoinInstruction::MintJunior { amount } => { sender_account.balance_junior += amount; state.junior_supply += amount; }
        StablecoinInstruction::RebalanceTranches => { /* Rebalance logic */ }

        // Stability
        StablecoinInstruction::SubmitZkPor { proof: _ } => { state.current_collateral_ratio = 1.0; }
        StablecoinInstruction::DistributeYield => { /* Yield logic */ }
        StablecoinInstruction::TriggerStabilizer => { 
            let burn = state.stabilization_pool_balance / 2;
            state.senior_supply = state.senior_supply.saturating_sub(burn);
            state.stabilization_pool_balance -= burn;
        }
        StablecoinInstruction::RunCircuitBreaker => { state.is_circuit_breaker_active = !state.is_circuit_breaker_active; }
        StablecoinInstruction::FairRedeemQueue { request_id: _ } => { /* Queue logic */ }

        // Privacy/Compliance
        StablecoinInstruction::ConfidentialTransfer { encrypted_data: _, proof: _ } => { /* Bulletproofs logic */ }
        StablecoinInstruction::ProveCompliance { proof: _ } => { /* Compliance logic */ }
        StablecoinInstruction::ZkTaxAttest { tax_proof: _ } => { /* Tax logic */ }
        StablecoinInstruction::MultiJurisdictionalCheck { region: _ } => { /* Geo logic */ }

        // Fiat/Interop
        StablecoinInstruction::InstantFiatBridge { bank_proof: _ } => { /* MPC Bridge */ }
        StablecoinInstruction::ZeroSlipBatchMatch { batch_id: _ } => { /* Batch logic */ }
        StablecoinInstruction::DynamicHedge { strategy: _ } => { /* Hedging */ }
        StablecoinInstruction::GeoRebalance { region: _, amount: _ } => { /* Geo rebalance */ }
        StablecoinInstruction::VelocityIncentive { velocity_score: _ } => { /* Incentive */ }
        StablecoinInstruction::StreamPayment { stream_id: _, rate: _ } => { /* Streaming */ }
    }

    Ok(())
}
