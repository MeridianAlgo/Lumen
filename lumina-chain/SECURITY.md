# Lumina Chain Security Model

## Overview

Lumina Chain is a high-security L1 blockchain designed for stablecoin issuance and confidential transfers.

## Threat Model & Attack Surface

### 1. Consensus Attacks (Malachite BFT)
- **Assumption**: > 2/3 of validators (by stake) are honest.
- **Attack**: Validator Cartel (>1/3).
  - **Impact**: Liveness failure (chain halt).
  - **Mitigation**: Slashing conditions for double-signing and unavailability.
- **Attack**: Byzantine General (>1/3).
  - **Impact**: Safety violation (forks).
  - **Mitigation**: BFT finality guarantees.

### 2. Stablecoin Mechanism Risks
- **Attack**: Oracle Manipulation.
  - **Impact**: Incorrect collateral valuation, leading to under-collateralized minting or unfair liquidations.
  - **Mitigation**: Multi-source weighted median oracle, time-weighted average price (TWAP), circuit breakers.
- **Attack**: Bank Run / Liquidity Crisis.
  - **Impact**: Senior tranche holders unable to redeem 1:1.
  - **Mitigation**: Dual-tranche architecture where Junior tranche absorbs first-loss. Dynamic redemption fees during high volatility.

### 3. Cryptographic Failures
- **Attack**: zk-PoR Soundness Breach.
  - **Impact**: Fake reserves.
  - **Mitigation**: Use of Groth16 (proven secure in ROM) and trusted setup ceremonies (Perpetual Powers of Tau).
- **Attack**: Discrete Log Break (Ed25519).
  - **Impact**: Key compromise.
  - **Mitigation**: Standard, battle-tested curves (Curve25519).

## Audit Checklist

- [ ] **Consensus**: Verify Malachite integration handles all edge cases (timeouts, view changes).
- [ ] **Economics**: Simulate death spiral scenarios for LJUN token.
- [ ] **Privacy**: Ensure Bulletproofs range proofs prevent negative balances (inflation bug).
- [ ] **Storage**: Verify Merkle Trie integrity under concurrent writes.
- [ ] **Network**: Fuzz test P2P message parsing (dos vectors).

## Formal Properties

- **Safety**: If a block is finalized, no conflicting block is ever finalized.
- **Liveness**: If the network is synchronous and honest majority exists, blocks are produced.
- **Soundness**: A valid zk-Proof implies knowledge of the witness with overwhelming probability.
