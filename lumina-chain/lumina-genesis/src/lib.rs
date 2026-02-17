use lumina_types::state::{GlobalState, AccountState};
use lumina_crypto::signatures::PublicKey;

pub fn build_genesis(initial_allocations: Vec<(PublicKey, u64)>) -> GlobalState {
    let mut state = GlobalState::default();
    
    for (pk, amount) in initial_allocations {
        let account = AccountState {
            balance_senior: amount,
            balance_junior: 0,
            nonce: 0,
        };
        state.accounts.insert(pk.to_bytes(), account);
        state.senior_supply += amount;
    }
    
    state
}
