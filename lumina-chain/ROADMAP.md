# LuminaChain Roadmap & Checklist

This document tracks the development progress of LuminaChain towards the "Ultimate 2026 Stablecoin Infrastructure".

## Phase 1: Core Foundation (Current Status)

- [x] **Project Structure**: workspace with 10+ crates (types, consensus, network, etc.)
- [x] **Cryptography**: Ed25519 signatures, Blake3 hashing, Arkworks Groth16 integration
- [x] **Data Structures**:
    - [x] Custom `Block` and `Transaction` structs
    - [x] `StablecoinInstruction` (SI) enum with 20+ variants
    - [x] Global State with dual-tranche balances (Senior/Junior)
- [x] **Execution Engine**:
    - [x] Deterministic `execute_si` state machine
    - [x] Handling of Mint/Redeem/Transfer
- [x] **Networking**:
    - [x] libp2p stack (Gossipsub + Noise + Yamux)
    - [x] Peer discovery and message propagation
- [x] **Consensus**:
    - [x] Malachite BFT integration (Mocked for local dev, structure ready for `circlefin/malachite`)
    - [x] Block proposal and commitment loop
- [x] **Storage**:
    - [x] RocksDB persistence
    - [x] Basic state serialization/deserialization
- [x] **API & Node**:
    - [x] `lumina-node` binary wiring all components
    - [x] REST API (Axum) for state queries and tx submission
    - [x] Docker Compose testnet setup

## Phase 2: Advanced Features (Next Steps)

- [ ] **Zero-Knowledge Circuits**:
    - [ ] Implement actual R1CS circuits for Proof-of-Reserves in `lumina-crypto`
    - [ ] Add confidential transfer circuits (Bulletproofs)
- [ ] **Economics & Stability**:
    - [ ] Implement `TriggerStabilizer` logic with real math
    - [ ] Add `FairRedeemQueue` to prevent bank runs
- [ ] **Production Hardening**:
    - [ ] Replace Mock Consensus with full Malachite Engine
    - [ ] Add proper Merkle Patricia Trie for state root calculation
    - [ ] Implement slashing conditions

## Phase 3: Ecosystem & Tooling

- [x] **CLI Wallet**: Basic keygen and minting commands
- [ ] **Explorer**: Simple web UI to view blocks
- [ ] **IBC**: Cross-chain bridge implementation

## Phase 4: Audit & Launch

- [ ] **Security Audit**: Third-party review of SIs and Circuits
- [ ] **Testnet**: Public incentivized testnet
- [ ] **Mainnet**: Genesis launch
