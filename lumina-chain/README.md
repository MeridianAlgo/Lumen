# LuminaChain (LUM)

LuminaChain is a high-performance, stablecoin-native L1 blockchain built from scratch in pure Rust.

## 🚀 Core Features

- **100% Rust**: Zero general-purpose VM by default for maximum security.
- **Stablecoin Native**: Business logic resides in native `StablecoinInstructions` (SIs).
- **Dual-Tranche Stability**: Senior (LUSD) and Junior (LJUN) tranches for risk absorption.
- **Malachite BFT**: Tendermint-based consensus for sub-second finality.
- **zk-Proof of Reserves**: Groth16 proofs for real-time collateral validation.
- **Confidential Transfers**: Pedersen commitments and Bulletproofs ready.
- **Lightweight**: Optimized for commodity hardware (<6GB RAM).

## 🛠️ Architecture

- `lumina-types`: Core data structures (Blocks, Txs, SIs).
- `lumina-execution`: Deterministic state machine and SI handlers.
- `lumina-consensus`: BFT consensus service.
- `lumina-network`: libp2p-based P2P stack.
- `lumina-crypto`: ZK circuits and signature schemes.
- `lumina-storage`: RocksDB persistence.
- `lumina-api`: REST API for node interaction.
- `lumina-cli`: Wallet management and CLI interaction.

## 🏁 Getting Started

### 1. Build
```bash
cargo build --release
```
*Note: Requires `libclang` for RocksDB/bindgen.*

### 2. Run Tests
```bash
cargo test
```

### 3. Launch Local Node
```bash
cargo run --release --bin lumina-node -- --data-dir ./data
```

### 4. CLI Wallet
```bash
# Init wallet
cargo run --bin lumina-cli -- init

# Mint LUSD
cargo run --bin lumina-cli -- mint --amount 1000 --asset lusd
```

## 📅 Roadmap
See `ROADMAP.md` for Phase details.

## ⚖️ License
MIT
