# Lumina Chain Developer Guide

## Table of Contents
1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Getting Started](#getting-started)
4. [Project Structure](#project-structure)
5. [Building from Source](#building-from-source)
6. [Running a Node](#running-a-node)
7. [Development Workflow](#development-workflow)
8. [Testing](#testing)
9. [Deployment](#deployment)
10. [Contributing](#contributing)

## Project Overview

Lumina Chain is a high-performance, production-grade Layer 1 blockchain built specifically for stablecoin operations. It's designed from the ground up to provide enterprise-grade stability, security, and performance for financial applications.

### Key Features
- **High Throughput**: 8,000+ TPS with sub-900ms finality
- **Dual-Token System**: LUSD (senior tranche) and LJUN (junior tranche)
- **Zero-Knowledge Proofs**: Privacy-preserving transactions with ZK-SNARKs
- **BFT Consensus**: Malachite BFT consensus for Byzantine fault tolerance
- **Enterprise Security**: Formal verification, memory safety, and audit trails

## Architecture

### System Architecture
```
┌─────────────────────────────────────────┐
│            Application Layer            │
│  (CLI, API, SDKs, Explorer)          │
├───────────────────────────────────────┤
│            Application Layer          │
│  (Stablecoin Engine, Oracles, DEX)   │
├───────────────────────────────────────┤
│            Execution Layer            │
│  (Stablecoin Instructions, ZK Proofs)│
├───────────────────────────────────────┤
│            Consensus Layer           │
│  (Malachite BFT, P2P, State Sync)   │
├───────────────────────────────────────┤
│            Storage Layer             │
│  (RocksDB, Merkle Trees, Snapshots)  │
└───────────────────────────────────────┘
```

### Core Components

#### 1. Consensus Layer (`lumina-consensus`)
- Malachite BFT consensus algorithm
- 2/3+1 Byzantine fault tolerance
- Sub-second block finality
- Validator set management

#### 2. Execution Layer (`lumina-execution`)
- Deterministic SI (Stablecoin Instruction) execution
- 50+ native stablecoin operations
- Zero-knowledge proof verification
- Gas metering and fee calculation

#### 3. Networking Layer (`lumina-network`)
- libp2p-based P2P networking
- GossipSub for block/transaction propagation
- Encrypted transport with TLS 1.3
- NAT traversal and hole punching

#### 4. Storage Layer (`lumina-storage`)
- RocksDB for persistent storage
- Merkle Patricia Trie for state
- Snapshot and checkpoint system
- Incremental state sync

#### 5. API Layer (`lumina-api`)
- RESTful JSON-RPC API
- gRPC interface for high-performance clients
- WebSocket subscriptions
- Rate limiting and DoS protection

## Getting Started

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install build dependencies
sudo apt-get update
sudo apt-get install -y clang cmake pkg-config libssl-dev

# Install protobuf compiler (for gRPC)
sudo apt-get install -y protobuf-compiler
```

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/luminachain/lumina.git
cd lumina-chain

# Build in release mode
cargo build --release --all-features

# Run tests
cargo test --all-features

# Run benchmarks
cargo bench
```

### Project Structure

```
lumina-chain/
├── Cargo.toml                    # Workspace manifest
├── Cargo.lock
├── README.md
├── lumina-types/                 # Core data types
│   ├── src/
│   │   ├── block.rs              # Block and transaction types
│   │   ├── instruction.rs        # Stablecoin instructions
│   │   ├── state.rs              # Global state types
│   │   └── transaction.rs        # Transaction types
│   └── Cargo.toml
├── lumina-crypto/                # Cryptography primitives
├── lumina-execution/             # Execution engine
├── lumina-consensus/            # Consensus layer
├── lumina-network/              # P2P networking
├── lumina-storage/              # Storage layer
├── lumina-api/                  # REST/gRPC API
├── lumina-cli/                  # Command line interface
├── lumina-node/                 # Main node binary
└── lumina-oracles/              # Oracle integration
```

## Building from Source

### Development Build

```bash
# Debug build with all features
cargo build --all-features

# Release build with optimizations
cargo build --release

# Build with specific features
cargo build --release --features "rocksdb,zk-snarks,malachite"
```

### Docker Build

```bash
# Build Docker image
docker build -t lumina-node:latest .

# Run in development mode
docker run -p 3000:3000 -p 4000:4000 lumina-node:latest
```

## Running a Node

### Local Development

```bash
# Start a single node with default settings
cargo run --bin lumina-node -- --data-dir ./data

# With custom configuration
cargo run --bin lumina-node -- \
  --data-dir ./data \
  --rpc-addr 0.0.0.0:3000 \
  --p2p-addr /ip4/0.0.0.0/tcp/4000 \
  --validator \
  --log-level info
```

### Configuration

Create a `config.toml`:

```toml
[network]
listen_addr = "/ip4/0.0.0.0/tcp/4000"
external_address = "/ip4/127.0.0.1/tcp/4000"
bootstrap_peers = [
    "/ip4/1.2.3.4/tcp/4000/p2p/12D3KooW...",
    "/ip4/5.6.7.8/tcp/4000/p2p/12D3KooW..."
]

[consensus]
timeout_propose = "3s"
timeout_commit = "1s"
timeout_prevote = "1s"
timeout_precommit = "1s"

[execution]
max_gas_per_block = 10000000
max_tx_size = 1048576
max_block_size = 1048576

[api]
listen_addr = "0.0.0.0:3000"
cors_origins = ["*"]
max_body_size = 10485760  # 10MB
```

### Genesis Configuration

```json
{
  "genesis_time": "2024-01-01T00:00:00Z",
  "chain_id": "lumina-1",
  "initial_height": "1",
  "app_hash": "",
  "initial_height": "1",
  "app_state": {
    "validators": [
      {
        "address": "luminavaloper1...",
        "pub_key": "luminavalconspub1...",
        "power": "1000000",
        "name": "genesis-validator"
      }
    ],
    "accounts": [
      {
        "address": "lumina1...",
        "coins": [
          {"denom": "ulumina", "amount": "1000000000"}
        ]
      }
    ]
  }
}
```

## Development Workflow

### Setting Up Development Environment

```bash
# Install development tools
cargo install cargo-watch
cargo install cargo-nextest  # Better test runner
cargo install cargo-udeps     # Check for unused dependencies

# Install pre-commit hooks
pre-commit install

# Install development dependencies
cargo install cargo-udeps cargo-audit cargo-tarpaulin
```

### Testing

```bash
# Run all tests
cargo test --all-features

# Run integration tests
cargo test --test integration

# Run with coverage
cargo tarpaulin --all-features

# Fuzz testing
cargo fuzz run fuzz_target
```

### Code Quality

```bash
# Format code
cargo fmt

# Linting
cargo clippy --all-features -- -D warnings

# Security audit
cargo audit
cargo deny check

# Performance benchmarks
cargo bench
```

## API Development

### REST API

The API follows RESTful conventions with JSON request/response format.

**Example: Query Account Balance**
```bash
curl -X GET "http://localhost:3000/accounts/lumina1.../balance"
```

**Example: Submit Transaction**
```bash
curl -X POST "http://localhost:3000/tx" \
  -H "Content-Type: application/json" \
  -d '{
    "sender": "lumina1...",
    "nonce": 42,
    "instruction": {
      "type": "Transfer",
      "to": "lumina1...",
      "amount": "1000000",
      "asset": "LUSD"
    },
    "signature": "0x..."
  }'
```

### gRPC API

For high-performance applications, use the gRPC interface:

```protobuf
service LuminaService {
  rpc GetBalance(GetBalanceRequest) returns (Balance) {}
  rpc SubmitTransaction(Transaction) returns (TransactionReceipt) {}
  rpc StreamBlocks(StreamBlocksRequest) returns (stream Block) {}
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use lumina_types::Transaction;
    
    #[test]
    fn test_transaction_validation() {
        let tx = Transaction::new();
        assert!(tx.validate().is_ok());
    }
}
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration

# With logging
RUST_LOG=debug cargo test -- --nocapture
```

### Performance Testing

```bash
# Run benchmarks
cargo bench

# Load testing
cargo run --bin load_test -- --tps 1000 --duration 60s
```

## Deployment

### Production Configuration

```yaml
# docker-compose.yml
version: '3.8'
services:
  lumina-node:
    image: luminachain/node:latest
    ports:
      - "3000:3000"  # API
      - "4000:4000"  # P2P
    volumes:
      - ./data:/data
    environment:
      - RUST_LOG=info
      - RUST_BACKTRACE=1
    command: >
      --data-dir /data
      --validator
      --rpc-addr 0.0.0.0:3000
      --p2p-addr /ip4/0.0.0.0/tcp/4000
```

### Monitoring

```bash
# Health check endpoint
curl http://localhost:3000/health

# Metrics endpoint (Prometheus format)
curl http://localhost:3000/metrics

# Node status
curl http://localhost:3000/status
```

### Logging and Monitoring

```rust
// Structured logging with tracing
tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .with_target(false)
    .with_thread_ids(true)
    .init();
```

## Security

### Key Management

```rust
// Generate keypair
let keypair = Keypair::new();
let public_key = keypair.public_key();
let signature = keypair.sign(message);
```

### Security Best Practices

1. **Never store private keys in source code**
2. Use hardware security modules (HSM) in production
3. Regular security audits and penetration testing
4. Follow principle of least privilege

### Auditing

```bash
# Security audit
cargo audit
cargo audit fix

# Dependency checking
cargo deny check
cargo outdated
```

## Performance Optimization

### Performance Tuning

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### Memory Management

```rust
// Use arenas for high-frequency allocations
let arena = Bump::new();
let data = arena.alloc(Data::new());
```

## Contributing

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Write tests for new features
4. Ensure all tests pass
5. Submit a pull request

### Code Style

```rust
// Use thiserror for error handling
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LuminaError {
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Insufficient balance")]
    InsufficientBalance,
}
```

### Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_transaction_serialization(tx in any::<Transaction>()) {
            let bytes = bincode::serialize(&tx).unwrap();
            let deserialized: Transaction = bincode::deserialize(&bytes).unwrap();
            assert_eq!(tx, deserialized);
        }
    }
}
```

## Troubleshooting

### Common Issues

1. **Port already in use**: Change port or kill existing process
2. **Insufficient funds**: Check account balance
3. **Network issues**: Verify P2P connectivity
4. **Storage corruption**: Delete data directory and resync

### Getting Help

- Check logs: `journalctl -u lumina-node`
- Enable debug logging: `RUST_LOG=debug`
- Check network connectivity: `netstat -tulpn`

## Additional Resources

- [API Documentation](https://docs.luminachain.com)
- [Whitepaper](https://whitepaper.luminachain.com)
- [Community Forum](https://forum.luminachain.com)
- [Discord Community](https://discord.gg/lumina)

## License

Lumina Chain is released under the MIT License. See LICENSE for details.

---

*This guide is a living document. Please contribute improvements via pull requests.*