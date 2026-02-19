"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { getSession, walletLogin } from "../lib/auth";
import { loadWallet } from "../lib/wallet";
import { bytesToHex } from "../lib/encoding";

export default function Page() {
  const router = useRouter();

  useEffect(() => {
    const wallet = loadWallet();
    const session = getSession();
    
    if (!wallet) {
      // No wallet - go to login to create/import
      router.replace("/auth/login");
      return;
    }
    
    // Wallet exists - ensure session and go to dashboard
    if (!session) {
      const address = "0x" + bytesToHex(wallet.publicKey);
      walletLogin(address, bytesToHex(wallet.publicKey));
    }
    
    router.replace("/dashboard");
  }, [router]);

  return (
    <div className="login-shell">
      <div className="login-container">
        <div className="logo" style={{ marginBottom: 20 }}>
          <svg viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg" width="60" height="60">
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
        <p style={{ color: "var(--muted)" }}>Connecting to Lumina...</p>
      </div>
    </div>
  );
}
