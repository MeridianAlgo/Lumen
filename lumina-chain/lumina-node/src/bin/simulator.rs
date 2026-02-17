use lumina_types::state::GlobalState;
use lumina_types::instruction::{StablecoinInstruction, TrancheType};
use lumina_execution::state_machine::execute_si;
use lumina_crypto::signatures::generate_keypair;
use ed25519_dalek::Signer;

fn main() {
    println!("--- LuminaChain Stress Simulator ---");
    let mut state = GlobalState::new();
    let admin_kp = generate_keypair();
    
    // 1. Genesis setup
    state.stabilization_pool_balance = 1_000_000;
    println!("Initial Stabilization Pool: {}", state.stabilization_pool_balance);

    // 2. Simulate User Mints
    let user1 = generate_keypair();
    let mint_tx = create_signed_tx(&user1, StablecoinInstruction::MintJunior { amount: 50_000 }, 1);
    execute_si(&mint_tx, &mut state).unwrap();
    println!("User1 minted 50k Junior. Junior Supply: {}", state.junior_supply);

    // 3. Simulate a Depeg
    state.current_collateral_ratio = 0.85;
    println!("CRITICAL: Collateral Ratio dropped to 0.85!");

    // 4. Trigger Stabilizer
    let stab_tx = create_signed_tx(&admin_kp, StablecoinInstruction::TriggerStabilizer, 1);
    execute_si(&stab_tx, &mut state).unwrap();
    println!("Stabilizer triggered. New Senior Supply: {}", state.senior_supply);

    // 5. Final State Report
    println!("--- Simulation Complete ---");
    println!("Final State: {:?}", state);
}

fn create_signed_tx(kp: &ed25519_dalek::SigningKey, instr: StablecoinInstruction, nonce: u64) -> lumina_types::transaction::Transaction {
    let mut tx = lumina_types::transaction::Transaction {
        instruction: instr,
        sender: kp.verifying_key().to_bytes(),
        nonce,
        signature: None,
    };
    let bytes = tx.signable_bytes();
    let sig = kp.sign(&bytes);
    tx.signature = Some(sig);
    tx
}
