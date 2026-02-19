export type WalletAuth = {
  address: string;
  publicKey: string;
  createdAt: number;
};

type Session = WalletAuth;

const SESSION_KEY = "lumina_wallet_session_v1";

function readJson<T>(key: string): T | null {
  if (typeof window === "undefined") return null;
  const raw = window.localStorage.getItem(key);
  if (!raw) return null;
  return JSON.parse(raw) as T;
}

function writeJson(key: string, value: unknown) {
  if (typeof window === "undefined") return;
  window.localStorage.setItem(key, JSON.stringify(value));
}

export function getSession(): Session | null {
  return readJson<Session>(SESSION_KEY);
}

export function requireSession(): Session {
  const s = getSession();
  if (!s) throw new Error("Wallet not connected");
  return s;
}

export function logout() {
  if (typeof window === "undefined") return;
  window.localStorage.removeItem(SESSION_KEY);
  window.localStorage.removeItem("lumina_wallet_v1");
}

export function walletLogin(address: string, publicKey: string) {
  const session: Session = { 
    address, 
    publicKey,
    createdAt: Date.now() 
  };
  writeJson(SESSION_KEY, session);
}
