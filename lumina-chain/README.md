# LuminaChain

The most secure, lightweight, stablecoin-native blockchain.

## Features
- Native Stablecoin Instructions (SIs) via enum-based VM
- Malachite BFT Consensus (Tendermint-based)
- libp2p Networking
- RocksDB Storage
- ZK-PoR & Compliance stubs (Arkworks)
- Pure Rust implementation

## Prerequisites
- Rust 1.75+
- Docker & Docker Compose
- Protobuf Compiler (`protoc`) for Malachite/Tonic

## Build
```bash
cargo build --release
```

## Run Testnet
```bash
docker-compose up --build
```

## CLI Usage
```bash
# Generate keypair
cargo run --bin lumina-cli keygen

# Mint LUM
cargo run --bin lumina-cli mint --amount 1000 --tranche senior
```

## Project Structure
- `lumina-types`: Core data structures (Block, Tx, SI)
- `lumina-execution`: State machine logic
- `lumina-consensus`: Consensus engine integration
- `lumina-network`: P2P networking
- `lumina-storage`: Database layer
- `lumina-node`: Main node binary
- `lumina-cli`: Wallet/Client tool
- `lumina-api`: REST/gRPC API
