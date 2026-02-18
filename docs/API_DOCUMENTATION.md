# Lumina Chain API Documentation

## Overview

Lumina Chain provides a comprehensive REST API for interacting with the blockchain network. The API is built using Axum and exposes endpoints for state queries, transaction submission, block retrieval, and system monitoring.

## Base URL

```
http://localhost:3000
```

For multi-node testnet deployments, each validator node exposes its API on a different port (3001-3007).

## Authentication

Currently, the API does not require authentication for read operations. Transaction submission requires valid cryptographic signatures embedded within the transaction payload.

## Endpoints

### 1. Root Endpoint

**GET /**  
Returns basic API information.

**Response Example:**
```json
"LuminaChain API v1.0 — Production L1 Stablecoin Network"
```

### 2. Global State

**GET /state**  
Returns a summary of the global blockchain state.

**Response Example Example:**
```json
{
  "total_lusd_supply": 1000000,
  "total_ljun_supply": 500000,
  "reserve_ratio": 1.05,
  "stabilization_pool_balance": 1050000,
  "circuit_breaker_active": false,
  "insurance_fund_balance": 50000,
  "health_index": 9500,
  "validator_count": 7,
  "custodian_count": 3,
  "rwa_listing_count": 12,
  "pending_redeem_queue": 0,
  "current_epoch": 42,
  "velocity_reward_pool": 100000,
  "account_count": 150
}
```

**Fields:**
- `total_lusd_supply`: Total LUSD (senior tranche) supply
- `total_ljun_supply`: Total LJUN (junior tranche) supply  
- `reserve_ratio`: Current reserve ratio (collateral / liabilities)
- `stabilization_pool_balance`: Total collateral in stabilization pool
- `circuit_breaker_active`: Whether circuit breaker is active
- `insurance_fund_balance`: Insurance fund balance
- `health_index`: Protocol health index (0-10000)
- `validator_count`: Number of active validators
- `custodian_count`: Number of registered custodians
- `rwa_listing_count`: Number of RWA listings
- `pending_redeem_queue`: Number of pending redemption requests
- `current_epoch`: Current velocity reward epoch
- `velocity_reward_pool`: Total rewards available for velocity incentives
- `account_count`: Total number of accounts

### 3. Health Status

**GET /health**  
Returns detailed health metrics for the protocol.

**Response Example:**
```json
{
  "health_index": 9500,
  "health_pct": "95.00%",
  "reserve_ratio": 1.05,
  "circuit_breaker_active": false,
  "insurance_fund_balance": 50000,
  "green_validator_count": 3,
  "pending_flash_mints": 0,
  "epoch_progress": "42/8640",
  "velocity_reward_pool": 100000
}
```

### 4. Submit Transaction

**POST /tx**  
Submit a new transaction to the mempool.

**Request Body:**
```json
{
  "sender": "0x...",
  "nonce": 42,
  "instruction": {
    "type": "Transfer",
    "to": "0x...",
    "amount": 1000,
    "asset": "LUSD"
  },
  "signature": "0x..."
}
```

**Response Example:**
```json
{
  "status": "accepted",
  "tx_hash": "0x...",
  "message": "Transaction added to mempool"
}
```

**Error Response Examples:**
- `400 Bad Request`: Invalid transaction format
- `403 Forbidden`: Invalid signature or insufficient balance
- `429 Too Many Requests`: Rate limit exceeded
- `503 Service Unavailable`: Circuit breaker active

### 5. Get Block by Height

**GET /block/{height}**  
Retrieve a block by its height.

**Path Parameters:**
- `height`: Block height (0 for genesis)

**Response Example:**
```json
{
  "header": {
    "height": 100,
    "prev_hash": "0x...",
    "transactions_root": "0x...",
    "state_root": "0x...",
    "timestamp": 1739800000,
    "proposer": "0x..."
  },
  "transactions": [
    {
      "sender": "0x...",
      "nonce": 1,
      "instruction": {...},
      "signature": "0x..."
    }
  ],
  "votes": [
    {
      "validator": "0x...",
      "signature": "0x..."
    }
  ],
  "hash": "0x..."
}
```

### 6. Get Account Information

**GET /account/{address}**  
Retrieve account state by address.

**Path Parameters:**
- `address`: Account address (hex encoded)

**Response Example:**
```json
{
  "address": "0x...",
  "nonce": 42,
  "balances": {
    "lusd": 10000,
    "ljun": 5000,
    "lumina": 1000000
  },
  "commitment": "0x...",
  "has_passkey": true,
  "guardian_count": 3,
  "has_pq_key": false,
  "credit_score": 750,
  "active_streams": 2,
  "yield_positions": 1,
  "epoch_tx_volume": 50000,
  "last_reward_epoch": 41
}
```

### 7. Faucet (Testnet Only)

**POST /faucet**  
Request test tokens from the faucet.

**Request Body:**
```json
{
  "address": "0x...",
  "asset": "LUSD",
  "amount": 1000
}
```

**Response Example:**
```json
{
  "status": "success",
  "tx_hash": "0x...",
  "message": "1000 LUSD minted to address 0x..."
}
```

**Note:** Only available on testnet. Requires valid address and reasonable amount.

### 8. Validator Information

**GET /validators**  
Retrieve list of active validators.

**Response Example:**
```json
{
  "validators": [
    {
      "pubkey": "0x...",
      "stake": 1000000,
      "power": 1000000,
      "is_green": true,
      "has_energy_proof": true
    }
  ],
  "total_stake": 7000000,
  "green_validator_count": 3,
  "green_validator_power": 3000000
}
```

### 9. Insurance Fund

**GET /insurance**  
Retrieve insurance fund information.

**Response Example:**
```json
{
  "balance": 50000,
  "total_claims": 0,
  "last_claim_height": 0,
  "funding_rate": 0.05,
  "coverage_ratio": 0.01
}
```

## Transaction Types

### Core Asset Operations
- `RegisterAsset`: Register new asset with ticker and decimals
- `MintSenior`: Mint LUSD with collateral
- `RedeemSenior`: Redeem LUSD for collateral
- `MintJunior`: Mint LJUN with collateral
- `RedeemJunior`: Redeem LJUN for collateral
- `Burn`: Burn tokens
- `Transfer`: Transfer tokens between accounts

### Stability & Tranche Management
- `RebalanceTranches`: Rebalance senior/junior tranches
- `DistributeYield`: Distribute yield to token holders
- `TriggerStabilizer`: Trigger stabilization mechanism
- `RunCircuitBreaker`: Activate/deactivate circuit breaker
- `FairRedeemQueue`: Process redemption queue

### Privacy & Compliance
- `ConfidentialTransfer`: Private transfer with ZK proof
- `ProveCompliance`: Submit compliance proof
- `ZkTaxAttest`: Submit tax attestation proof
- `MultiJurisdictionalCheck`: Multi-jurisdiction compliance check

### Oracle & Reserves
- `UpdateOracle`: Update oracle price
- `SubmitZkPoR`: Submit zero-knowledge proof of reserves

### Advanced DeFi & Fiat Hooks
- `InstantFiatBridge`: Bridge to fiat with MPC signature
- `ZeroSlipBatchMatch`: Zero-slip batch order matching
- `DynamicHedge`: Dynamic hedging operation
- `GeoRebalance`: Geographic rebalancing
- `VelocityIncentive`: Velocity incentive adjustment
- `StreamPayment`: Create streaming payment

### Governance & Staking
- `RegisterValidator`: Register as validator
- `Vote`: Vote on governance proposal

### Security & Recovery
- `CreatePasskeyAccount`: Create passkey-based account
- `RecoverSocial`: Social recovery operation
- `ClaimVelocityReward`: Claim velocity rewards
- `RegisterCustodian`: Register as custodian
- `RotateReserves`: Rotate reserve custodians
- `ClaimInsurance`: Claim from insurance fund

### Advanced Features
- `SwitchToPQSignature`: Switch to post-quantum signatures
- `RegisterGreenValidator`: Register as green validator
- `UploadComplianceCircuit`: Upload compliance circuit
- `FlashMint`: Flash mint operation
- `FlashBurn`: Flash burn operation
- `MintWithCreditScore`: Mint with credit score proof
- `WrapToYieldToken`: Wrap to yield token
- `UnwrapYieldToken`: Unwrap yield token
- `ListRWA`: List real-world asset
- `CollateralizeRWA`: Collateralize RWA
- `ComputeHealthIndex`: Compute health index

## Error Codes

| Code | Description | Resolution |
|------|-------------|------------|
| 400 | Bad Request | Check request format and parameters |
| 401 | Unauthorized | Valid signature required |
| 403 | Forbidden | Insufficient balance or permissions |
| 404 | Not Found | Resource not found |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Node error, try again later |
| 503 | Service Unavailable | Circuit breaker active or maintenance |

## Rate Limiting

- Read endpoints: 100 requests per minute per IP
- Write endpoints: 10 requests per minute per IP
- Faucet: 1 request per hour per address

## WebSocket Events

Coming soon: Real-time WebSocket API for:
- New block notifications
- Transaction confirmations
- State change events
- Oracle price updates

## SDKs and Client Libraries

### Rust
```rust
use lumina_api::Client;

let client = Client::new("http://localhost:3000");
let state = client.get_state().await?;
```

### JavaScript/TypeScript
```typescript
import { LuminaClient } from '@lumina-chain/sdk';

const client = new LuminaClient('http://localhost:3000');
const state = await client.getState();
```

### Python
```python
from lumina_sdk import LuminaClient

client = LuminaClient("http://localhost:3000")
state = client.get_state()
```

## Best Practices

1. **Transaction Nonces**: Always use sequential nonces starting from 0
2. **Error Handling**: Check error Response Examples and implement retry logic
3. **Signature Verification**: Verify signatures before submission
4. **Gas Estimation**: Include sufficient Lumina for transaction fees
5. **State Consistency**: Use block height for consistent state queries
6. **Circuit Breaker**: Check circuit breaker status before critical operations

## Versioning

API version is included in the root endpoint Response Example. Breaking changes will increment the major version.

## Support

For API issues or questions:
- Check node logs for detailed error information
- Verify transaction format and signatures
- Ensure sufficient balance for operations
- Monitor circuit breaker status during stress periods