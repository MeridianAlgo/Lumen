# Lumina Chain

Lumina Chain is a high-performance, stablecoin-native L1 blockchain built in Rust. It features a novel dual-tranche stablecoin mechanism (LUSD/LJUN), zk-Proof of Reserves, and privacy-preserving transactions.

## Architecture

- **Consensus**: Malachite BFT (Tendermint-based) with sub-second finality.
- **Network**: libp2p (Gossipsub + Noise + Yamux).
- **Execution**: Pure Rust State Machine with deterministic Stablecoin Instructions (SIs).
- **Storage**: RocksDB with Merkle Trie (stateless validation ready).
- **Privacy**: Zero-Knowledge Proofs (Groth16) and Confidential Transfers (Bulletproofs).

## Requirements

- Rust 1.75+
- Clang (for RocksDB)
- CMake

## Build

```bash
cargo build --release
```

## Run Local Testnet

```bash
# Initialize data directory
cargo run --bin lumina-node -- --data-dir ./data

# Run validator node
cargo run --bin lumina-node -- --validator --data-dir ./validator1
```

## Docker Testnet

Start a 7-validator testnet with simulated oracle and fiat bridge:

```bash
docker-compose up --build
```

## API

- **Get State**: `GET http://localhost:3000/state`
- **Submit Tx**: `POST http://localhost:3000/tx`
- **Faucet**: `POST http://localhost:3000/faucet`

## License

MIT
