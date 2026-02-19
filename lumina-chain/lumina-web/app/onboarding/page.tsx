"use client";

import { useRouter } from "next/navigation";
import { useEffect, useMemo, useState } from "react";
import { bytesToHex, hexToBytes } from "../../lib/encoding";
import { logout, walletLogin } from "../../lib/auth";
import { loadWallet, newWallet, saveWallet } from "../../lib/wallet";
import type { Wallet } from "../../lib/types";

export default function OnboardingPage() {
  const router = useRouter();
  const [wallet, setWallet] = useState<Wallet | null>(null);
  const [importSkHex, setImportSkHex] = useState("");
  const [showSecret, setShowSecret] = useState(false);
  const [toast, setToast] = useState<{ kind: "ok" | "err"; msg: string } | null>(null);
  const [busy, setBusy] = useState(false);

  // Redirect to dashboard if wallet already exists
  useEffect(() => {
    const w = loadWallet();
    if (w) {
      const address = "0x" + bytesToHex(w.publicKey);
      walletLogin(address, bytesToHex(w.publicKey));
      router.replace("/dashboard");
    }
  }, [router]);

  const addressHex = useMemo(() => {
    if (!wallet) return null;
    return "0x" + bytesToHex(wallet.publicKey);
  }, [wallet]);

  async function doCreate() {
    setBusy(true);
    setToast(null);
    try {
      const w = newWallet();
      const addr = "0x" + bytesToHex(w.publicKey);
      const pk = bytesToHex(w.publicKey);
      
      saveWallet(w);
      walletLogin(addr, pk);
      setWallet(w);
      setToast({ kind: "ok", msg: "Wallet created! Save your secret key securely." });
    } catch (e: any) {
      setToast({ kind: "err", msg: e?.message ?? String(e) });
    } finally {
      setBusy(false);
    }
  }

  async function doImport() {
    setBusy(true);
    setToast(null);
    try {
      const raw = importSkHex.trim().toLowerCase().replace(/^0x/, "");
      const sk = hexToBytes(raw);
      const w = newWallet(sk);
      const addr = "0x" + bytesToHex(w.publicKey);
      const pk = bytesToHex(w.publicKey);
      
      saveWallet(w);
      walletLogin(addr, pk);
      setWallet(w);
      setImportSkHex("");
      setToast({ kind: "ok", msg: "Wallet imported successfully!" });
    } catch (e: any) {
      setToast({ kind: "err", msg: e?.message ?? String(e) });
    } finally {
      setBusy(false);
    }
  }

  async function doContinue() {
    if (!wallet) {
      setToast({ kind: "err", msg: "Create or import a wallet first." });
      return;
    }
    router.replace("/dashboard");
  }

  async function doLogout() {
    logout();
    router.replace("/auth/login");
  }

  return (
    <div className="login-shell">
      <div className="onboarding-container">
        <div className="onboarding-header">
          <div className="logo">
            <svg viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
              <circle cx="20" cy="20" r="18" stroke="url(#grad1)" strokeWidth="2"/>
              <path d="M12 20L18 26L28 14" stroke="url(#grad1)" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"/>
              <defs>
                <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stopColor="#7c5cff"/>
                  <stop offset="100%" stopColor="#35c27a"/>
                </linearGradient>
              </defs>
            </svg>
          </div>
          <h1>Welcome to Lumina</h1>
          <p>Create or import your wallet to get started</p>
        </div>

        <div className="onboarding-grid">
          <div className={`onboarding-card ${wallet ? 'success' : ''}`}>
            <div className="card-badge">{wallet ? '✓' : '1'}</div>
            <h3>{wallet ? 'Wallet Ready' : 'Create Wallet'}</h3>
            
            {!wallet ? (
              <>
                <p className="card-desc">
                  Generate a new Ed25519 wallet. Your address is your public key.
                </p>
                <button 
                  className={`login-btn primary ${busy ? 'loading' : ''}`}
                  onClick={doCreate}
                  disabled={busy}
                >
                  <span className="btn-content">
                    <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
                      <path d="M10 4v12M4 10h12" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                    </svg>
                    Generate Wallet
                  </span>
                </button>
              </>
            ) : (
              <>
                <div className="wallet-preview">
                  <div className="preview-row">
                    <span>Address</span>
                    <code>{addressHex?.slice(0, 16)}...{addressHex?.slice(-8)}</code>
                  </div>
                </div>
                <button className="login-btn primary" onClick={doContinue}>
                  <span className="btn-content">Enter Dashboard →</span>
                </button>
              </>
            )}
          </div>

          <div className="onboarding-card">
            <div className="card-badge">2</div>
            <h3>Import Wallet</h3>
            <p className="card-desc">
              Already have a wallet? Paste your secret key to restore it.
            </p>
            
            <div className="import-wrapper">
              <input
                className="modern-input"
                type={showSecret ? "text" : "password"}
                value={importSkHex}
                onChange={(e) => setImportSkHex(e.target.value)}
                placeholder="Secret key (hex)..."
                disabled={busy || !!wallet}
              />
              <button
                className="toggle-visibility"
                onClick={() => setShowSecret(!showSecret)}
                type="button"
              >
                {showSecret ? "🙈" : "👁️"}
              </button>
            </div>
            
            <button 
              className={`login-btn secondary ${busy ? 'loading' : ''}`}
              onClick={doImport}
              disabled={!importSkHex || busy || !!wallet}
            >
              <span className="btn-content">
                <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
                  <path d="M4 10h12M10 4l6 6-6 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                </svg>
                Import
              </span>
            </button>
          </div>

          {wallet && (
            <div className="onboarding-card full-width warning">
              <div className="card-badge">!</div>
              <h3>Backup Your Keys</h3>
              <p className="card-desc">
                This is a development wallet stored in browser localStorage. 
                Copy your secret key now — it will not be shown again.
              </p>
              <div className="secret-box">
                <code>{bytesToHex(wallet.secretKey)}</code>
                <button 
                  className="copy-btn"
                  onClick={() => navigator.clipboard.writeText(bytesToHex(wallet.secretKey))}
                >
                  Copy
                </button>
              </div>
            </div>
          )}
        </div>

        {toast && (
          <div className={`toast-float ${toast.kind}`}>
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              {toast.kind === 'ok' ? (
                <path d="M8 0a8 8 0 100 16A8 8 0 008 0zm3.5 6.5L7 11l-2.5-2.5 1-1L7 9l3.5-3.5 1 1z"/>
              ) : (
                <path d="M8 0a8 8 0 100 16A8 8 0 008 0zm0 12a1 1 0 110-2 1 1 0 010 2zm0-3a1 1 0 01-.995-.89L7 8V4a1 1 0 012 0v4l-.005.11A1 1 0 018 9z"/>
              )}
            </svg>
            {toast.msg}
          </div>
        )}

        <div className="onboarding-footer">
          <button className="text-link" onClick={doLogout}>
            ← Back to login
          </button>
        </div>
      </div>
    </div>
  );
}
