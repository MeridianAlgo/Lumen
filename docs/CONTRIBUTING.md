# Contributing to Lumina Chain

## Welcome

Thank you for your interest in contributing to Lumina Chain! This document provides guidelines and instructions for contributing to the project.

## Table of Contents
1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Code Standards](#code-standards)
5. [Testing](#testing)
6. [Documentation](#documentation)
7. [Security](#security)
8. [Pull Request Process](#pull-request-process)
9. [Community](#community)

## Code of Conduct

### Our Pledge
We pledge to make participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards
Examples of behavior that contributes to creating a positive environment include:
- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

Examples of unacceptable behavior include:
- The use of sexualized language or imagery
- Trolling, insulting/derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information without explicit permission
- Other conduct which could reasonably be considered inappropriate

### Enforcement
Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the project team at conduct@luminachain.com. All complaints will be reviewed and investigated promptly and fairly.

## Getting Started

### Prerequisites
- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Git
- Basic understanding of blockchain concepts

### Setting Up Development Environment

```bash
# 1. Fork the repository
# Visit https://github.com/luminachain/lumina and click "Fork"

# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/lumina.git
cd lumina-chain

# 3. Add upstream remote
git remote add upstream https://github.com/luminachain/lumina.git

# 4. Install development tools
cargo install cargo-watch
cargo install cargo-nextest
cargo install cargo-udeps
cargo install cargo-audit
cargo install cargo-tarpaulin

# 5. Build the project
cargo build --all-features
```

### Project Structure
```
lumina-chain/
├── Cargo.toml                    # Workspace manifest
├── Cargo.lock
├── README.md
├── CONTRIBUTING.md
├── lumina-types/                 # Core data types
├── lumina-crypto/                # Cryptography
├── lumina-execution/             # Execution engine
├── lumina-consensus/             # Consensus layer
├── lumina-network/              # Networking
├── lumina-storage/              # Storage
├── lumina-api/                  # API layer
├── lumina-cli/                  # CLI
├── lumina-node/                 # Main node
├── lumina-genesis/              # Genesis
├── lumina-oracles/              # Oracles
└── tests/                       # Test suites
```

## Development Workflow

### Branch Strategy
- `main`: Production-ready code
- `develop`: Integration branch
- `feature/*`: New features
- `bugfix/*`: Bug fixes
- `release/*`: Release preparation
- `hotfix/*`: Critical fixes

### Creating a Feature Branch
```bash
# Sync with upstream
git fetch upstream
git checkout develop
git rebase upstream/develop

# Create feature branch
git checkout -b feature/your-feature-name

# Make your changes
# ...

# Commit with descriptive message
git commit -m "feat: add new transaction type"
```

### Commit Message Convention
We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(consensus): add timeout configuration
fix(storage): resolve data corruption issue
docs(api): update endpoint documentation
test(execution): add unit tests for SI execution
```

## Code Standards

### Rust Code Style
We follow Rust community standards:

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

// Use anyhow for application errors
use anyhow::{Result, Context};

pub fn process_transaction(tx: Transaction) -> Result<()> {
    tx.validate()
        .context("Failed to validate transaction")?;
    Ok(())
}

// Use proper documentation
/// Process a transaction and update state.
///
/// # Arguments
/// * `tx` - The transaction to process
/// * `state` - Current blockchain state
///
/// # Returns
/// Result containing updated state or error
pub fn execute(tx: Transaction, state: &mut State) -> Result<()> {
    // Implementation
}
```

### Code Quality Tools
```bash
# Format code
cargo fmt

# Lint code
cargo clippy --all-features -- -D warnings

# Check for unused dependencies
cargo udeps

# Security audit
cargo audit

# Check for outdated dependencies
cargo outdated
```

### Performance Considerations
```rust
// Use appropriate data structures
use std::collections::{HashMap, BTreeMap};

// For frequent lookups: HashMap
let mut accounts: HashMap<Address, Account> = HashMap::new();

// For ordered iteration: BTreeMap
let mut validators: BTreeMap<Address, Validator> = BTreeMap::new();

// Use references to avoid cloning
pub fn process(&self, data: &[u8]) -> Result<()> {
    // Process data without cloning
}

// Use efficient serialization
use bincode::{serialize, deserialize};
let bytes = serialize(&data)?;
let decoded: Data = deserialize(&bytes)?;
```

## Testing

### Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_basic_functionality() {
        // Unit test
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    #[should_panic(expected = "overflow")]
    fn test_overflow() {
        // Test that panics
        u8::MAX + 1;
    }
    
    proptest! {
        #[test]
        fn test_serialization_roundtrip(tx in any::<Transaction>()) {
            // Property-based test
            let bytes = bincode::serialize(&tx).unwrap();
            let deserialized: Transaction = bincode::deserialize(&bytes).unwrap();
            assert_eq!(tx, deserialized);
        }
    }
}
```

### Running Tests
```bash
# Run all tests
cargo test --all-features

# Run specific test suite
cargo test --test integration

# Run with coverage
cargo tarpaulin --all-features

# Run benchmarks
cargo bench

# Fuzz testing
cargo fuzz run transaction_fuzz
```

### Integration Tests
```bash
# Set up test environment
./scripts/setup-testnet.sh

# Run integration tests
cargo test --test integration -- --test-threads=1

# Clean up
./scripts/cleanup-testnet.sh
```

## Documentation

### Code Documentation
```rust
/// A block in the Lumina Chain.
///
/// Blocks contain transactions and are linked together
/// to form the blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block header containing metadata
    pub header: BlockHeader,
    
    /// List of transactions in this block
    pub transactions: Vec<Transaction>,
    
    /// Validator votes for this block
    pub votes: Vec<Vote>,
}

impl Block {
    /// Create a new block with the given header and transactions.
    ///
    /// # Arguments
    /// * `header` - Block header
    /// * `transactions` - List of transactions
    ///
    /// # Returns
    /// New Block instance
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self {
            header,
            transactions,
            votes: Vec::new(),
        }
    }
}
```

### API Documentation
```rust
/// REST API endpoint for submitting transactions.
///
/// # Endpoint
/// POST /tx
///
/// # Request Body
/// ```json
/// {
///   "tx": "base64_encoded_transaction",
///   "mode": "sync"
/// }
/// ```
///
/// # Response
/// ```json
/// {
///   "code": 0,
///   "data": "base64_data",
///   "hash": "0x..."
/// }
/// ```
pub async fn submit_transaction(
    State(state): State<AppState>,
    Json(request): Json<SubmitTxRequest>,
) -> Result<Json<SubmitTxResponse>, ApiError> {
    // Implementation
}
```

## Security

### Security Best Practices
1. **Never commit secrets**: Use environment variables or config files
2. **Validate all inputs**: Sanitize and validate user input
3. **Use safe arithmetic**: Prevent overflow/underflow
4. **Implement proper error handling**: Don't expose internal errors
5. **Follow principle of least privilege**: Minimal permissions required

### Security Review Process
```bash
# Run security checks
cargo audit
cargo deny check

# Check for unsafe code
cargo geiger

# Dependency analysis
cargo tree
cargo outdated
```

### Reporting Security Issues
If you discover a security vulnerability, please report it responsibly:
1. **DO NOT** create a public issue
2. Email security@luminachain.com
3. Include detailed description and steps to reproduce
4. We will acknowledge within 24 hours
5. We will work with you to fix and disclose

## Pull Request Process

### Creating a Pull Request
1. **Fork the repository**
2. **Create a feature branch**
3. **Make your changes**
4. **Write tests**
5. **Update documentation**
6. **Run all checks**
7. **Submit PR**

### PR Checklist
- [ ] Code follows project standards
- [ ] Tests pass
- [ ] Documentation updated
- [ ] No security issues
- [ ] Performance considered
- [ ] Backward compatibility maintained

### PR Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update
- [ ] Other (please describe)

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manual testing performed

## Documentation
- [ ] Code comments added/updated
- [ ] API documentation updated
- [ ] README updated if needed

## Security
- [ ] Security audit performed
- [ ] No new vulnerabilities introduced

## Additional Notes
Any additional information
```

### Review Process
1. **Automated checks** (CI/CD pipeline)
2. **Code review** by maintainers
3. **Security review** if needed
4. **Performance review** for critical changes
5. **Merge approval** from 2+ maintainers

## Community

### Communication Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and discussions
- **Discord**: Real-time chat and support
- **Forum**: In-depth technical discussions
- **Twitter**: Announcements and updates

### Getting Help
```bash
# Check existing issues
https://github.com/luminachain/lumina/issues

# Search documentation
https://docs.luminachain.com

# Join Discord
https://discord.gg/lumina

# Ask on forum
https://forum.luminachain.com
```

### Recognition
We recognize contributors through:
- **GitHub contributors list**
- **Release notes acknowledgments**
- **Community spotlight features**
- **Swag and merchandise**
- **Conference invitations**

## Governance

### Decision Making
- **Technical decisions**: Technical steering committee
- **Community decisions**: Community voting
- **Security decisions**: Security team
- **Release decisions**: Release managers

### Roles and Responsibilities
- **Maintainers**: Code review, release management
- **Contributors**: Code contributions, bug fixes
- **Reviewers**: Code review, quality assurance
- **Documenters**: Documentation updates
- **Testers**: Testing and validation

## License

By contributing to Lumina Chain, you agree that your contributions will be licensed under the project's MIT License.

## Acknowledgments

We appreciate all contributions, big and small. Thank you for helping make Lumina Chain better!

---

*This document is maintained by the Lumina Chain team. Last updated: February 2026*