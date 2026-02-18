# Lumina Chain API Reference

## Overview

Lumina Chain provides a comprehensive REST API for interacting with the blockchain. This document details all available endpoints, request/response formats, and usage examples.

## Base URLs

- **Mainnet**: `https://api.luminachain.com`
- **Testnet**: `https://testnet-api.luminachain.com`
- **Local**: `http://localhost:3000`

## Authentication

### API Keys
```bash
# Include API key in header
curl -H "X-API-Key: your-api-key" https://api.luminachain.com/status
```

### JWT Tokens
```bash
# Include JWT token
curl -H "Authorization: Bearer your-jwt-token" https://api.luminachain.com/status
```

## Rate Limits

| Tier | Requests per Second | Burst | Monthly Requests |
|------|---------------------|-------|------------------|
| Free | 10 | 20 | 100,000 |
| Pro | 100 | 200 | 1,000,000 |
| Enterprise | 1,000 | 2,000 | 10,000,000 |

## Endpoints

### Health and Status

#### GET /health
Get health status of the node.

**Response:**
```json
{
  "status": "healthy",
  "version": "2.1.0",
  "chain_id": "lumina-1",
  "block_height": 123456,
  "sync_status": "synced",
  "validators": 42,
  "peers": 15,
  "uptime": "7d 12h 30m",
  "timestamp": "2026-02-17T10:30:00Z"
}
```

#### GET /status
Get detailed node status.

**Response:**
```json
{
  "node_info": {
    "id": "12D3KooW...",
    "listen_addr": "0.0.0.0:26656",
    "network": "lumina-1",
    "version": "2.1.0",
    "channels": "40202122233038606100",
    "moniker": "validator-01",
    "protocol_version": {
      "p2p": "8",
      "block": "11",
      "app": "0"
    }
  },
  "sync_info": {
    "latest_block_hash": "A5C7...",
    "latest_app_hash": "B3D9...",
    "latest_block_height": "123456",
    "latest_block_time": "2026-02-17T10:30:00Z",
    "earliest_block_hash": "0000...",
    "earliest_app_hash": "0000...",
    "earliest_block_height": "1",
    "earliest_block_time": "2026-01-01T00:00:00Z",
    "catching_up": false
  },
  "validator_info": {
    "address": "luminavaloper1...",
    "pub_key": "luminavalconspub1...",
    "voting_power": "1000000"
  }
}
```

### Blockchain Data

#### GET /blocks/{height}
Get block by height.

**Path Parameters:**
- `height`: Block height (integer)

**Response:**
```json
{
  "block_id": {
    "hash": "A5C7...",
    "parts": {
      "total": 1,
      "hash": "B3D9..."
    }
  },
  "block": {
    "header": {
      "version": {
        "block": "11",
        "app": "0"
      },
      "chain_id": "lumina-1",
      "height": "123456",
      "time": "2026-02-17T10:30:00Z",
      "last_block_id": {
        "hash": "C2E8...",
        "parts": {
          "total": 1,
          "hash": "D4F6..."
        }
      },
      "last_commit_hash": "E1A9...",
      "data_hash": "F7B2...",
      "validators_hash": "G5C3...",
      "next_validators_hash": "H8D4...",
      "consensus_hash": "I6E5...",
      "app_hash": "J9F7...",
      "last_results_hash": "K0G8...",
      "evidence_hash": "L1H9...",
      "proposer_address": "luminavalcons1..."
    },
    "data": {
      "txs": [
        "base64_encoded_tx_1",
        "base64_encoded_tx_2"
      ]
    },
    "evidence": {
      "evidence": []
    },
    "last_commit": {
      "height": "123455",
      "round": 0,
      "block_id": {
        "hash": "C2E8...",
        "parts": {
          "total": 1,
          "hash": "D4F6..."
        }
      },
      "signatures": [
        {
          "block_id_flag": 2,
          "validator_address": "luminavalcons1...",
          "timestamp": "2026-02-17T10:29:55Z",
          "signature": "base64_signature"
        }
      ]
    }
  }
}
```

#### GET /blocks/latest
Get latest block.

**Response:** Same as `/blocks/{height}`

#### GET /blocks/{minHeight}/{maxHeight}
Get blocks in range.

**Path Parameters:**
- `minHeight`: Minimum block height
- `maxHeight`: Maximum block height

**Response:**
```json
{
  "blocks": [
    { /* block 1 */ },
    { /* block 2 */ }
  ],
  "total": 2,
  "next": "/blocks/123458/123468"
}
```

### Transactions

#### GET /tx/{hash}
Get transaction by hash.

**Path Parameters:**
- `hash`: Transaction hash (hex)

**Response:**
```json
{
  "hash": "0x1234...",
  "height": 123456,
  "index": 0,
  "tx_result": {
    "code": 0,
    "data": "base64_data",
    "log": "[{\"msg_index\":0,\"events\":[{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"send\"}]}]}]",
    "info": "",
    "gas_wanted": "200000",
    "gas_used": "100000",
    "events": [
      {
        "type": "message",
        "attributes": [
          {
            "key": "action",
            "value": "send"
          }
        ]
      }
    ],
    "codespace": ""
  },
  "tx": "base64_encoded_tx"
}
```

#### POST /tx
Broadcast transaction.

**Request Body:**
```json
{
  "tx": "base64_encoded_tx",
  "mode": "sync"  // "sync", "async", or "block"
}
```

**Response:**
```json
{
  "code": 0,
  "data": "base64_data",
  "log": "[]",
  "hash": "0x1234...",
  "height": 123456
}
```

#### GET /txs
Search transactions.

**Query Parameters:**
- `message.action`: Action type
- `message.sender`: Sender address
- `transfer.recipient`: Recipient address
- `page`: Page number (default: 1)
- `limit`: Results per page (default: 100, max: 1000)

**Response:**
```json
{
  "total_count": 1500,
  "count": 100,
  "page_number": 1,
  "page_total": 15,
  "limit": 100,
  "txs": [
    { /* transaction 1 */ },
    { /* transaction 2 */ }
  ]
}
```

### Accounts

#### GET /accounts/{address}
Get account information.

**Path Parameters:**
- `address`: Account address

**Response:**
```json
{
  "address": "lumina1...",
  "account_number": "42",
  "sequence": "100",
  "pub_key": {
    "@type": "/cosmos.crypto.secp256k1.PubKey",
    "key": "base64_pubkey"
  },
  "balances": [
    {
      "denom": "ulumina",
      "amount": "1000000000"
    },
    {
      "denom": "ibc/...",
      "amount": "500000000"
    }
  ]
}
```

#### GET /accounts/{address}/balance
Get account balance.

**Response:**
```json
{
  "balances": [
    {
      "denom": "ulumina",
      "amount": "1000000000"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```

#### GET /accounts/{address}/transactions
Get account transactions.

**Query Parameters:**
- `page`: Page number
- `limit`: Results per page
- `type`: Transaction type

**Response:**
```json
{
  "total": 150,
  "page": 1,
  "limit": 50,
  "transactions": [
    { /* transaction 1 */ },
    { /* transaction 2 */ }
  ]
}
```

### Validators

#### GET /validators
Get all validators.

**Query Parameters:**
- `status`: "bonded", "unbonding", "unbonded"
- `page`: Page number
- `limit`: Results per page

**Response:**
```json
{
  "validators": [
    {
      "operator_address": "luminavaloper1...",
      "consensus_pubkey": {
        "@type": "/cosmos.crypto.ed25519.PubKey",
        "key": "base64_pubkey"
      },
      "jailed": false,
      "status": "BOND_STATUS_BONDED",
      "tokens": "1000000000",
      "delegator_shares": "1000000000.000000000000000000",
      "description": {
        "moniker": "validator-01",
        "identity": "",
        "website": "https://validator.com",
        "security_contact": "",
        "details": "Professional validator"
      },
      "unbonding_height": "0",
      "unbonding_time": "1970-01-01T00:00:00Z",
      "commission": {
        "commission_rates": {
          "rate": "0.100000000000000000",
          "max_rate": "0.200000000000000000",
          "max_change_rate": "0.010000000000000000"
        },
        "update_time": "2026-01-01T00:00:00Z"
      },
      "min_self_delegation": "1"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "42"
  }
}
```

#### GET /validators/{validatorAddr}
Get validator by address.

**Response:** Single validator object

#### GET /validators/{validatorAddr}/delegations
Get validator delegations.

**Response:**
```json
{
  "delegation_responses": [
    {
      "delegation": {
        "delegator_address": "lumina1...",
        "validator_address": "luminavaloper1...",
        "shares": "1000000.000000000000000000"
      },
      "balance": {
        "denom": "ulumina",
        "amount": "1000000"
      }
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```

### Staking

#### GET /staking/delegators/{delegatorAddr}/delegations
Get delegator delegations.

**Response:**
```json
{
  "delegation_responses": [
    {
      "delegation": {
        "delegator_address": "lumina1...",
        "validator_address": "luminavaloper1...",
        "shares": "1000000.000000000000000000"
      },
      "balance": {
        "denom": "ulumina",
        "amount": "1000000"
      }
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```

#### GET /staking/delegators/{delegatorAddr}/unbonding_delegations
Get delegator unbonding delegations.

**Response:**
```json
{
  "unbonding_responses": [
    {
      "delegator_address": "lumina1...",
      "validator_address": "luminavaloper1...",
      "entries": [
        {
          "creation_height": "123000",
          "completion_time": "2026-02-24T10:30:00Z",
          "initial_balance": "1000000",
          "balance": "1000000"
        }
      ]
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```

#### GET /staking/delegators/{delegatorAddr}/validators
Get delegator validators.

**Response:** List of validators

### Governance

#### GET /gov/proposals
Get governance proposals.

**Query Parameters:**
- `proposal_status`: "PROPOSAL_STATUS_UNSPECIFIED", "PROPOSAL_STATUS_DEPOSIT_PERIOD", "PROPOSAL_STATUS_VOTING_PERIOD", "PROPOSAL_STATUS_PASSED", "PROPOSAL_STATUS_REJECTED", "PROPOSAL_STATUS_FAILED"
- `voter`: Voter address
- `depositor`: Depositor address

**Response:**
```json
{
  "proposals": [
    {
      "proposal_id": "1",
      "content": {
        "@type": "/cosmos.gov.v1beta1.TextProposal",
        "title": "Increase block gas limit",
        "description": "Proposal to increase block gas limit from 10M to 20M"
      },
      "status": "PROPOSAL_STATUS_VOTING_PERIOD",
      "final_tally_result": {
        "yes": "5000000",
        "abstain": "100000",
        "no": "200000",
        "no_with_veto": "0"
      },
      "submit_time": "2026-02-10T10:30:00Z",
      "deposit_end_time": "2026-02-17T10:30:00Z",
      "total_deposit": [
        {
          "denom": "ulumina",
          "amount": "1000000"
        }
      ],
      "voting_start_time": "2026-02-17T10:30:00Z",
      "voting_end_time": "2026-02-24T10:30:00Z"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```

#### GET /gov/proposals/{proposalId}
Get proposal by ID.

**Response:** Single proposal object

#### GET /gov/proposals/{proposalId}/votes
Get proposal votes.

**Response:**
```json
{
  "votes": [
    {
      "proposal_id": "1",
      "voter": "lumina1...",
      "option": "VOTE_OPTION_YES",
      "options": [
        {
          "option": "VOTE_OPTION_YES",
          "weight": "1.000000000000000000"
        }
      ]
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```

### Bank

#### GET /bank/balances/{address}
Get bank balances.

**Response:**
```json
{
  "balances": [
    {
      "denom": "ulumina",
      "amount": "1000000000"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```

#### GET /bank/total
Get total supply.

**Response:**
```json
{
  "supply": [
    {
      "denom": "ulumina",
      "amount": "1000000000000"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```

### Distribution

#### GET /distribution/delegators/{delegatorAddr}/rewards
Get delegator rewards.

**Response:**
```json
{
  "rewards": [
    {
      "validator_address": "luminavaloper1...",
      "reward": [
        {
          "denom": "ulumina",
          "amount": "1000.000000000000000000"
        }
      ]
    }
  ],
  "total": [
    {
      "denom": "ulumina",
      "amount": "1000.000000000000000000"
    }
  ]
}
```

#### GET /distribution/delegators/{delegatorAddr}/withdraw_address
Get delegator withdraw address.

**Response:**
```json
{
  "withdraw_address": "lumina1..."
}
```

### Slashing

#### GET /slashing/validators/{validatorAddr}/signing_info
Get validator signing info.

**Response:**
```json
{
  "val_signing_info": {
    "address": "luminavalcons1...",
    "start_height": "0",
    "index_offset": "123456",
    "jailed_until": "1970-01-01T00:00:00Z",
    "tombstoned": false,
    "missed_blocks_counter": "0"
  }
}
```

### Mint

#### GET /minting/inflation
Get current inflation.

**Response:**
```json
{
  "inflation": "0.070000000000000000"
}
```

#### GET /minting/annual-provisions
Get annual provisions.

**Response:**
```json
{
  "annual_provisions": "70000000.000000000000000000"
}
```

### IBC

#### GET /ibc/apps/transfer/v1/denom_traces
Get IBC denom traces.

**Response:**
```json
{
  "denom_traces": [
    {
      "path": "transfer/channel-0",
      "base_denom": "ulumina"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```

### Auth

#### GET /auth/accounts/{address}
Get auth account.

**Response:**
```json
{
  "account": {
    "@type": "/cosmos.auth.v1beta1.BaseAccount",
    "address": "lumina1...",
    "pub_key": {
      "@type": "/cosmos.crypto.secp256k1.PubKey",
      "key": "base64_pubkey"
    },
    "account_number": "42",
    "sequence": "100"
  }
}
```

### Tendermint

#### GET /tendermint/validatorsets/{height}
Get validator set at height.

**Response:**
```json
{
  "block_height": "123456",
  "validators": [
    {
      "address": "luminavalcons1...",
      "pub_key": {
        "@type": "/cosmos.crypto.ed25519.PubKey",
        "key": "base64_pubkey"
      },
      "voting_power": "1000000",
      "proposer_priority": "0"
    }
  ]
}
```

#### GET /tendermint/validatorsets/latest
Get latest validator set.

**Response:** Same as above

## WebSocket API

### Connect to WebSocket
```javascript
const ws = new WebSocket('wss://api.luminachain.com/websocket');

ws.onopen = () => {
  // Subscribe to events
  ws.send(JSON.stringify({
    jsonrpc: "2.0",
    method: "subscribe",
    id: 1,
    params: {
      query: "tm.event='NewBlock'"
    }
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('New block:', data);
};
```

### Available Subscriptions

#### New Blocks
```json
{
  "jsonrpc": "2.0",
  "method": "subscribe",
  "id": 1,
  "params": {
    "query": "tm.event='NewBlock'"
  }
}
```

#### Transactions
```json
{
  "jsonrpc": "2.0",
  "method": "subscribe",
  "id": 2,
  "params": {
    "query": "tm.event='Tx'"
  }
}
```

#### Validator Updates
```json
{
  "jsonrpc": "2.0",
  "method": "subscribe",
  "id": 3,
  "params": {
    "query": "tm.event='ValidatorSetUpdates'"
  }
}
```

## gRPC API

### Protobuf Definitions
```protobuf
service Query {
  // Account returns account details based on address
  rpc Account(QueryAccountRequest) returns (QueryAccountResponse) {}
  
  // Balance returns balance of a denomination for a given account
  rpc Balance(QueryBalanceRequest) returns (QueryBalanceResponse) {}
  
  // AllBalances returns all balances for a given account
  rpc AllBalances(QueryAllBalancesRequest) returns (QueryAllBalancesResponse) {}
}
```

### gRPC Endpoints
- **Mainnet**: `grpc.luminachain.com:9090`
- **Testnet**: `grpc.testnet.luminachain.com:9090`

## Error Codes

### HTTP Status Codes
| Code | Description |
|------|-------------|
| 200 | Success |
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 429 | Too Many Requests |
| 500 | Internal Server Error |
| 503 | Service Unavailable |

### Application Error Codes
| Code | Description |
|------|-------------|
| 1 | Internal Error |
| 2 | Tx Decode Error |
| 3 | Invalid Sequence |
| 4 | Unauthorized |
| 5 | Insufficient Funds |
| 6 | Unknown Request |
| 7 | Invalid Address |
| 8 | Invalid Pubkey |
| 9 | Unknown Address |
| 10 | Invalid Coins |
| 11 | Out of Gas |
| 12 | Memo Too Large |
| 13 | Insufficient Fee |
| 14 | Too Many Signatures |
| 15 | No Signatures |

## SDK Examples

### JavaScript/TypeScript
```typescript
import { LuminaClient } from '@lumina-chain/sdk';

const client = new LuminaClient({
  apiUrl: 'https://api.luminachain.com',
  apiKey: 'your-api-key'
});

// Get account balance
const balance = await client.getAccountBalance('lumina1...');

// Send transaction
const tx = await client.sendTokens({
  from: 'lumina1...',
  to: 'lumina1...',
  amount: '1000000',
  denom: 'ulumina'
});
```

### Python
```python
from lumina_sdk import LuminaClient

client = LuminaClient(api_url='https://api.luminachain.com', api_key='your-api-key')

# Get latest block
block = client.get_latest_block()

# Query transactions
txs = client.search_transactions(sender='lumina1...')
```

### Go
```go
package main

import (
    "context"
    "github.com/luminachain/go-sdk/client"
)

func main() {
    c := client.NewClient("https://api.luminachain.com")
    
    // Get node status
    status, err := c.GetStatus(context.Background())
    if err != nil {
        panic(err)
    }
}
```

## Best Practices

### 1. Error Handling
```javascript
try {
  const response = await fetch('/api/endpoint');
  if (!response.ok) {
    throw new Error(`HTTP ${response.status}: ${response.statusText}`);
  }
  const data = await response.json();
} catch (error) {
  console.error('API Error:', error);
  // Implement retry logic
}
```

### 2. Rate Limiting
```javascript
// Implement exponential backoff
async function fetchWithRetry(url, retries = 3) {
  for (let i = 0; i < retries; i++) {
    try {
      return await fetch(url);
    } catch (error) {
      if (error.status === 429) {
        await new Promise(resolve => 
          setTimeout(resolve, Math.pow(2, i) * 1000)
        );
      } else {
        throw error;
      }
    }
  }
}
```

### 3. Caching
```javascript
// Cache responses when appropriate
const cache = new Map();

async function getCached(endpoint) {
  if (cache.has(endpoint)) {
    return cache.get(endpoint);
  }
  
  const data = await fetch(endpoint);
  cache.set(endpoint, data);
  return data;
}
```

## Versioning

API version is included in the response headers:
```
X-API-Version: 2.1.0
```

Breaking changes will increment the major version (e.g., 2.x → 3.x).

## Support

For API issues:
- Documentation: docs.luminachain.com/api
- Support: support@luminachain.com
- Status: status.luminachain.com
- GitHub Issues: github.com/luminachain/lumina/issues

---

*Last Updated: February 2026*  
*API Version: 2.1.0*