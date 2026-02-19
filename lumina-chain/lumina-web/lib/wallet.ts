import nacl from "tweetnacl";

import { hexToBytes, bytesToHex } from "./encoding";
import type { Wallet } from "./types";

const KEY = "lumina_wallet_v1";

type StoredWallet = {
  publicKeyHex: string;
  secretKeyHex: string;
};

export function newWallet(seedOrSecretKey?: Uint8Array): Wallet {
  if (!seedOrSecretKey || seedOrSecretKey.length === 0) {
    const kp = nacl.sign.keyPair();
    return {
      publicKey: kp.publicKey,
      secretKey: kp.secretKey
    };
  }

  if (seedOrSecretKey.length === 32) {
    const kp = nacl.sign.keyPair.fromSeed(seedOrSecretKey);
    return { publicKey: kp.publicKey, secretKey: kp.secretKey };
  }

  if (seedOrSecretKey.length === 64) {
    const sk = seedOrSecretKey;
    const pk = sk.slice(32, 64);
    return { publicKey: pk, secretKey: sk };
  }

  throw new Error("Secret key must be 32-byte seed or 64-byte secret key");
}

export function sign(wallet: Wallet, message: Uint8Array): Uint8Array {
  const sig = nacl.sign.detached(message, wallet.secretKey);
  return sig;
}

export function saveWallet(wallet: Wallet | null) {
  if (typeof window === "undefined") return;
  if (!wallet) {
    window.localStorage.removeItem(KEY);
    return;
  }
  const stored: StoredWallet = {
    publicKeyHex: bytesToHex(wallet.publicKey),
    secretKeyHex: bytesToHex(wallet.secretKey)
  };
  window.localStorage.setItem(KEY, JSON.stringify(stored));
}

export function loadWallet(): Wallet | null {
  if (typeof window === "undefined") return null;
  const raw = window.localStorage.getItem(KEY);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw) as StoredWallet;
    return {
      publicKey: hexToBytes(parsed.publicKeyHex),
      secretKey: hexToBytes(parsed.secretKeyHex)
    };
  } catch {
    return null;
  }
}
