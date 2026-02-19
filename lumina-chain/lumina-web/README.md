# lumina-web

Next.js web wallet + feature testing console for LuminaChain.

## Prereqs

- A running `lumina-api` (Axum) on `http://localhost:3000`.

## Run

From `lumina-chain/lumina-web`:

1. Install deps

```bash
npm install
```

2. Start dev server

```bash
npm run dev
```

Open `http://localhost:3001` if Next picks that port (or whatever it prints).

## How tx signing works

JavaScript does **not** reimplement Rust `bincode`.

Instead the app:

1. Builds an unsigned transaction payload.
2. Calls `POST /tx/signing_bytes` on `lumina-api`.
3. Signs the returned bytes with Ed25519 (`tweetnacl`).
4. Submits the signed tx to `POST /tx`.

## Notes

- Your address is your 32-byte Ed25519 public key.
- `FlashMint` requires `FlashBurn` of the full amount in the same block.
