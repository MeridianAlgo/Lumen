#[cfg(test)]
mod tests {
    use crate::state_machine::execute_si;
    use lumina_types::state::GlobalState;
    use lumina_types::transaction::Transaction;
    use lumina_types::instruction::{StablecoinInstruction, TrancheType};
    use lumina_crypto::signatures::generate_keypair;
    use ed25519_dalek::Signer;

    fn create_signed_tx(kp: &ed25519_dalek::SigningKey, instr: StablecoinInstruction, nonce: u64) -> Transaction {
        let mut tx = Transaction {
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

    #[test]
    fn test_basic_transfer() {
        let mut state = GlobalState::new();
        let kp1 = generate_keypair();
        let kp2 = generate_keypair();
        
        // Setup initial balance
        let pk1 = kp1.verifying_key().to_bytes();
        state.accounts.insert(pk1, lumina_types::state::AccountState {
            balance_senior: 1000,
            ..Default::default()
        });
        
        let tx = create_signed_tx(&kp1, StablecoinInstruction::Transfer { 
            to: kp2.verifying_key().to_bytes(), 
            amount: 400 
        }, 1);
        
        execute_si(&tx, &mut state).expect("Transfer failed");
        
        assert_eq!(state.accounts.get(&pk1).unwrap().balance_senior, 600);
        assert_eq!(state.accounts.get(&kp2.verifying_key().to_bytes()).unwrap().balance_senior, 400);
    }

    #[test]
    fn test_stabilizer_logic() {
        let mut state = GlobalState::new();
        state.senior_supply = 10000;
        state.stabilization_pool_balance = 2000;
        state.current_collateral_ratio = 0.8; // Depegged
        
        let kp = generate_keypair();
        let tx = create_signed_tx(&kp, StablecoinInstruction::TriggerStabilizer, 1);
        
        execute_si(&tx, &mut state).expect("Stabilizer failed");
        
        // 2000 / 2 = 1000 burned
        assert_eq!(state.senior_supply, 9000);
        assert_eq!(state.stabilization_pool_balance, 1000);
    }
}
