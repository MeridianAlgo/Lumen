use super::*;
use lumina_types::state::{GlobalState, AccountState};
use lumina_types::instruction::{StablecoinInstruction, AssetType};
use lumina_types::transaction::Transaction;

#[test]
fn test_mint_senior() {
    let mut state = GlobalState::default();
    let sender = [1u8; 32];
    
    // Setup initial state
    state.accounts.insert(sender, AccountState::default());

    let mut ctx = ExecutionContext {
        state: &mut state,
        height: 1,
        timestamp: 100,
    };

    let instruction = StablecoinInstruction::MintSenior {
        amount: 500,
        collateral_amount: 500,
        proof: vec![],
    };

    let tx = Transaction {
        sender,
        nonce: 0,
        instruction,
        signature: vec![],
        gas_limit: 1000,
        gas_price: 1,
    };

    // Execute
    assert!(execute_transaction(&tx, &mut ctx).is_ok());

    // Verify State
    let account = state.accounts.get(&sender).unwrap();
    assert_eq!(account.lusd_balance, 500);
    assert_eq!(state.total_lusd_supply, 500);
}

#[test]
fn test_transfer() {
    let mut state = GlobalState::default();
    let sender = [1u8; 32];
    let receiver = [2u8; 32];
    
    state.accounts.insert(sender, AccountState {
        nonce: 0,
        lusd_balance: 1000,
        ljun_balance: 0,
        lumina_balance: 1000,
    });

    let mut ctx = ExecutionContext {
        state: &mut state,
        height: 1,
        timestamp: 100,
    };

    let instruction = StablecoinInstruction::Transfer {
        to: receiver,
        amount: 200,
        asset: AssetType::LUSD,
    };

    let tx = Transaction {
        sender,
        nonce: 0,
        instruction,
        signature: vec![],
        gas_limit: 1000,
        gas_price: 1,
    };

    assert!(execute_transaction(&tx, &mut ctx).is_ok());

    let sender_acc = state.accounts.get(&sender).unwrap();
    let receiver_acc = state.accounts.get(&receiver).unwrap();

    assert_eq!(sender_acc.lusd_balance, 800);
    assert_eq!(receiver_acc.lusd_balance, 200);
}
