# LuminaChain Security Model

## 1. Threat Model & Attack Surface
- **Consensus:** Malachite BFT handles <33% Byzantine faults.
- **Execution:** No general VM = no reentrancy, no gas limit exploits. SIs are atomic.
- **Networking:** Libp2p with Noise encryption prevents eavesdropping.

## 2. Cryptography
- **Signatures:** Ed25519 (Dalek).
- **Hashing:** BLAKE3 (collision resistance).
- **ZK-PoR:** Arkworks Groth16 for proof of reserves.

## 3. Compliance & Privacy
- **Confidential Transfers:** Bulletproofs (stubbed).
- **Travel Rule:** Encrypted metadata attached to transactions.

## 4. Audit Checklist
- [ ] Verify all `unsafe` blocks (none used in core logic).
- [ ] Fuzz testing for SIs.
- [ ] Formal verification of Malachite integration.
