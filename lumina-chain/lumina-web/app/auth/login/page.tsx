"use client";

import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { getSession, walletLogin } from "../../../lib/auth";
import { hexToBytes, bytesToHex } from "../../../lib/encoding";
import { newWallet, saveWallet } from "../../../lib/wallet";

export default function LoginPage() {
  const router = useRouter();
  const [importSkHex, setImportSkHex] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [isImporting, setIsImporting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showSecret, setShowSecret] = useState(false);

  useEffect(() => {
    if (getSession()) router.replace("/dashboard");
  }, [router]);

  async function createWallet() {
    setIsCreating(true);
    setError(null);
    
    try {
      const wallet = newWallet();
      const address = "0x" + bytesToHex(wallet.publicKey);
      const publicKeyHex = bytesToHex(wallet.publicKey);
      
      saveWallet(wallet);
      walletLogin(address, publicKeyHex);
      
      router.replace("/dashboard");
    } catch (e: any) {
      setError(e?.message ?? "Failed to create wallet");
      setIsCreating(false);
    }
  }

  async function importWallet() {
    setIsImporting(true);
    setError(null);

    try {
      const raw = importSkHex.trim().toLowerCase().replace(/^0x/, "");
      const sk = hexToBytes(raw);
      const wallet = newWallet(sk);
      const address = "0x" + bytesToHex(wallet.publicKey);
      const publicKeyHex = bytesToHex(wallet.publicKey);
      
      saveWallet(wallet);
      walletLogin(address, publicKeyHex);
      
      router.replace("/dashboard");
    } catch (e: any) {
      setError(e?.message ?? "Invalid secret key");
      setIsImporting(false);
    }
  }

  return (
    <div className="login-shell">
      <div className="login-container">
        <div className="login-card">
          <div className="brand-section">
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
            <h1 className="brand-title">Lumina</h1>
            <p className="brand-subtitle">Your wallet is your identity</p>
          </div>

          <div className="login-options">
            <button 
              className={`login-btn primary ${isCreating ? 'loading' : ''}`}
              onClick={createWallet}
              disabled={isCreating || isImporting}
            >
              {isCreating ? (
                <span className="btn-content">
                  <span className="spinner"></span>
                  Creating...
                </span>
              ) : (
                <span className="btn-content">
                  <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
                    <path d="M10 4v12M4 10h12" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                  </svg>
                  Create New Wallet
                </span>
              )}
            </button>

            <div className="divider">
              <span>or</span>
            </div>

            <div className="import-section">
              <label className="input-label">Import Existing Wallet</label>
              <div className="input-wrapper">
                <input
                  className="modern-input"
                  type={showSecret ? "text" : "password"}
                  value={importSkHex}
                  onChange={(e) => setImportSkHex(e.target.value)}
                  placeholder="Paste secret key (hex)..."
                  disabled={isCreating || isImporting}
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
                className={`login-btn secondary ${isImporting ? 'loading' : ''}`}
                onClick={importWallet}
                disabled={!importSkHex || isCreating || isImporting}
              >
                {isImporting ? (
                  <span className="btn-content">
                    <span className="spinner"></span>
                    Importing...
                  </span>
                ) : (
                  <span className="btn-content">
                    <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
                      <path d="M4 10h12M10 4l6 6-6 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                    </svg>
                    Import Wallet
                  </span>
                )}
              </button>
            </div>

            {error && (
              <div className="error-message">
                <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                  <path d="M8 0a8 8 0 100 16A8 8 0 008 0zm0 12a1 1 0 110-2 1 1 0 010 2zm0-3a1 1 0 01-.995-.89L7 8V4a1 1 0 012 0v4l-.005.11A1 1 0 018 9z"/>
                </svg>
                {error}
              </div>
            )}
          </div>

          <div className="security-notice">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 1l2.5 2.5h3.5v3.5L16 8l-1.5 1.5v3.5h-3.5L8 15l-2.5-2.5H2v-3.5L.5 8 2 6.5V3h3.5L8 1zM6 8l2 2 4-4-1-1-3 3-1-1-1 1z"/>
            </svg>
            <span>Your keys never leave your browser. Stored securely in localStorage.</span>
          </div>
        </div>

        <div className="feature-preview">
          <div className="preview-card">
            <h3>Explore Lumina Chain</h3>
            <ul className="feature-list">
              <li>
                <span className="feature-icon">⚡</span>
                <span className="feature-text">
                  <strong>Flash Mint</strong>
                  <small>Instant liquidity with same-block burn</small>
                </span>
              </li>
              <li>
                <span className="feature-icon">🏛️</span>
                <span className="feature-text">
                  <strong>RWA Marketplace</strong>
                  <small>Real world assets as collateral</small>
                </span>
              </li>
              <li>
                <span className="feature-icon">📊</span>
                <span className="feature-text">
                  <strong>Credit Scoring</strong>
                  <small>On-chain credit for better rates</small>
                </span>
              </li>
              <li>
                <span className="feature-icon">🔐</span>
                <span className="feature-text">
                  <strong>Quantum Ready</strong>
                  <small>Post-quantum signature support</small>
                </span>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}
