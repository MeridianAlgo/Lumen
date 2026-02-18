use lumina_types::instruction::{StablecoinInstruction, AssetType};
use lumina_types::state::{GlobalState, AccountState};
use lumina_types::transaction::Transaction;
use anyhow::{Result, bail};

pub struct ExecutionContext<'a> {
    pub state: &'a mut GlobalState,
    pub height: u64,
    pub timestamp: u64,
}

pub fn execute_transaction(tx: &Transaction, ctx: &mut ExecutionContext) -> Result<()> {
    // 1. Verify Signature (simplified for now)
    // verify_signature(&tx)?;

    // 2. Check Nonce
    let sender_account = ctx.state.accounts.entry(tx.sender).or_default();
    if tx.nonce != sender_account.nonce {
         // Allow for 0 nonce on first tx
         if tx.nonce != 0 || sender_account.nonce != 0 {
            bail!("Invalid nonce: expected {}, got {}", sender_account.nonce, tx.nonce);
         }
    }
    sender_account.nonce += 1;

    // 3. Deduct Gas (simplified)
    // let gas_cost = tx.gas_limit * tx.gas_price;
    // if sender_account.lumina_balance < gas_cost { bail!("Insufficient gas"); }
    // sender_account.lumina_balance -= gas_cost;

    // 4. Execute Instruction
    execute_si(&tx.instruction, &tx.sender, ctx)
}

pub fn execute_si(si: &StablecoinInstruction, sender: &[u8; 32], ctx: &mut ExecutionContext) -> Result<()> {
    match si {
        StablecoinInstruction::MintSenior { amount, collateral_amount, proof: _ } => {
            // Verify proof (mock for now)
            // In real impl: verify_zk_proof(proof, collateral_amount)?;

            // Update state
            let account = ctx.state.accounts.entry(*sender).or_default();
            // Assume bridge deposit happened and we are minting against it
            // Logic: Collateral locked -> Mint LUSD
            account.lusd_balance += amount;
            ctx.state.total_lusd_supply += amount;
            Ok(())
        },
        StablecoinInstruction::RedeemSenior { amount } => {
            let account = ctx.state.accounts.entry(*sender).or_default();
            if account.lusd_balance < *amount {
                bail!("Insufficient LUSD balance");
            }
            account.lusd_balance -= amount;
            ctx.state.total_lusd_supply -= amount;
            // Trigger collateral release logic
            Ok(())
        },
        StablecoinInstruction::MintJunior { amount, collateral_amount: _ } => {
             let account = ctx.state.accounts.entry(*sender).or_default();
             account.ljun_balance += amount;
             ctx.state.total_ljun_supply += amount;
             Ok(())
        },
        StablecoinInstruction::RedeemJunior { amount } => {
            let account = ctx.state.accounts.entry(*sender).or_default();
            if account.ljun_balance < *amount {
                bail!("Insufficient LJUN balance");
            }
            account.ljun_balance -= amount;
            ctx.state.total_ljun_supply -= amount;
            Ok(())
        },
        StablecoinInstruction::Transfer { to, amount, asset } => {
             let sender_account = ctx.state.accounts.entry(*sender).or_default();
             match asset {
                 AssetType::LUSD => {
                     if sender_account.lusd_balance < *amount { bail!("Insufficient LUSD"); }
                     sender_account.lusd_balance -= amount;
                     let receiver = ctx.state.accounts.entry(*to).or_default();
                     receiver.lusd_balance += amount;
                 },
                 AssetType::LJUN => {
                     if sender_account.ljun_balance < *amount { bail!("Insufficient LJUN"); }
                     sender_account.ljun_balance -= amount;
                     let receiver = ctx.state.accounts.entry(*to).or_default();
                     receiver.ljun_balance += amount;
                 },
                 AssetType::Lumina(val) => {
                     // Handle native token transfer logic if different
                     if sender_account.lumina_balance < *val { bail!("Insufficient Lumina"); }
                     sender_account.lumina_balance -= val;
                     let receiver = ctx.state.accounts.entry(*to).or_default();
                     receiver.lumina_balance += val;
                 }
             }
             Ok(())
        },
        StablecoinInstruction::UpdateOracle { asset, price, timestamp: _, signature: _ } => {
            // Verify signature from authorized oracle set
            ctx.state.oracle_prices.insert(asset.clone(), *price);
            Ok(())
        },
        StablecoinInstruction::RegisterValidator { pubkey, stake } => {
            let account = ctx.state.accounts.entry(*sender).or_default();
            // Lock stake
            // Update validator set
             ctx.state.validators.push(lumina_types::state::ValidatorState {
                 pubkey: *pubkey,
                 stake: *stake,
                 power: *stake, // Simplified power = stake
             });
            Ok(())
        },
        _ => {
            // Placeholder for other SIs
            Ok(())
        }
    }
}
