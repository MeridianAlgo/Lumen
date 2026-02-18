use lumina_types::instruction::{StablecoinInstruction, AssetType};
use lumina_types::state::{GlobalState, AccountState, RedemptionRequest};
use lumina_types::transaction::Transaction;
use anyhow::{Result, bail};

pub struct ExecutionContext<'a> {
    pub state: &'a mut GlobalState,
    pub height: u64,
    pub timestamp: u64,
}

pub fn execute_transaction(tx: &Transaction, ctx: &mut ExecutionContext) -> Result<()> {
    let sender_account = ctx.state.accounts.entry(tx.sender).or_default();
    if tx.nonce != sender_account.nonce {
         if tx.nonce != 0 || sender_account.nonce != 0 {
            bail!("Invalid nonce: expected {}, got {}", sender_account.nonce, tx.nonce);
         }
    }
    sender_account.nonce += 1;
    execute_si(&tx.instruction, &tx.sender, ctx)
}

pub fn execute_si(si: &StablecoinInstruction, sender: &[u8; 32], ctx: &mut ExecutionContext) -> Result<()> {
    match si {
        StablecoinInstruction::MintSenior { amount, .. } => {
            if ctx.state.circuit_breaker_active { bail!("Circuit breaker active"); }
            let account = ctx.state.accounts.entry(*sender).or_default();
            account.lusd_balance += amount;
            ctx.state.total_lusd_supply += amount;
            recalculate_ratios(ctx);
            Ok(())
        },
        StablecoinInstruction::RedeemSenior { amount } => {
            let account = ctx.state.accounts.entry(*sender).or_default();
            if account.lusd_balance < *amount { bail!("Insufficient LUSD"); }
            if ctx.state.circuit_breaker_active || ctx.state.reserve_ratio < 0.95 {
                 ctx.state.fair_redeem_queue.push(RedemptionRequest {
                     address: *sender,
                     amount: *amount,
                     timestamp: ctx.timestamp,
                 });
                 account.lusd_balance -= amount;
                 return Ok(());
            }
            account.lusd_balance -= amount;
            ctx.state.total_lusd_supply -= amount;
            recalculate_ratios(ctx);
            Ok(())
        },
        StablecoinInstruction::MintJunior { amount, .. } => {
            let account = ctx.state.accounts.entry(*sender).or_default();
            account.ljun_balance += amount;
            ctx.state.total_ljun_supply += amount;
            recalculate_ratios(ctx);
            Ok(())
        },
        StablecoinInstruction::RedeemJunior { amount } => {
            let account = ctx.state.accounts.entry(*sender).or_default();
            if account.ljun_balance < *amount { bail!("Insufficient LJUN"); }
            account.ljun_balance -= amount;
            ctx.state.total_ljun_supply -= amount;
            recalculate_ratios(ctx);
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
                     if sender_account.lumina_balance < *val { bail!("Insufficient Lumina"); }
                     sender_account.lumina_balance -= val;
                     let receiver = ctx.state.accounts.entry(*to).or_default();
                     receiver.lumina_balance += val;
                 },
                 _ => bail!("Asset type not supported for transfer"),
             }
             Ok(())
        },
        StablecoinInstruction::TriggerStabilizer => {
            if ctx.state.reserve_ratio < 1.0 && ctx.state.stabilization_pool_balance > 0 {
                let deficit = (ctx.state.total_lusd_supply as f64 * (1.0 - ctx.state.reserve_ratio)) as u64;
                let amount_to_move = std::cmp::min(deficit, ctx.state.stabilization_pool_balance);
                ctx.state.stabilization_pool_balance -= amount_to_move;
                ctx.state.reserve_ratio += (amount_to_move as f64 / ctx.state.total_lusd_supply as f64);
            }
            Ok(())
        },
        StablecoinInstruction::DistributeYield { total_yield } => {
            // Yield goes to Junior tranche (risk absorbers)
            if ctx.state.total_ljun_supply > 0 {
                 // In a real system, we'd update a yield index or distribute to a pool
                 // For now, we mock the logic of increasing the LJUN "backing"
            }
            Ok(())
        },
        StablecoinInstruction::InstantFiatBridge { amount, .. } => {
            // Logic for MPC sig verification omitted (Phase 4 item)
            // Mock bridge: increase stabilization pool balance
            ctx.state.stabilization_pool_balance += amount;
            recalculate_ratios(ctx);
            Ok(())
        },
        StablecoinInstruction::UpdateOracle { asset, price, .. } => {
            ctx.state.oracle_prices.insert(asset.clone(), *price);
            recalculate_ratios(ctx);
            Ok(())
        },
        StablecoinInstruction::RunCircuitBreaker { active } => {
            ctx.state.circuit_breaker_active = *active;
            Ok(())
        },
        StablecoinInstruction::FairRedeemQueue { batch_size } => {
            if ctx.state.circuit_breaker_active { bail!("Circuit breaker active: Cannot process queue"); }
            let to_process = std::cmp::min(*batch_size as usize, ctx.state.fair_redeem_queue.len());
            for _ in 0..to_process {
                let req = ctx.state.fair_redeem_queue.remove(0);
                ctx.state.total_lusd_supply -= req.amount;
                // In a real system, release collateral here
            }
            recalculate_ratios(ctx);
            Ok(())
        },
        StablecoinInstruction::RegisterValidator { pubkey, stake } => {
            ctx.state.validators.push(lumina_types::state::ValidatorState {
                pubkey: *pubkey,
                stake: *stake,
                power: *stake,
            });
            Ok(())
        },
        _ => Ok(()), // Default to NOP for unimplemented instructions
    }
}

fn recalculate_ratios(ctx: &mut ExecutionContext) {
    if ctx.state.total_lusd_supply == 0 {
        ctx.state.reserve_ratio = 1.0;
        return;
    }
    let eth_price = ctx.state.oracle_prices.get("ETH-USD").unwrap_or(&3000_000_000);
    ctx.state.reserve_ratio = (*eth_price as f64 / 3000_000_000.0) + (ctx.state.stabilization_pool_balance as f64 / ctx.state.total_lusd_supply as f64);
    if ctx.state.reserve_ratio < 0.85 {
        ctx.state.circuit_breaker_active = true;
    }
}

#[cfg(test)]
mod tests;
