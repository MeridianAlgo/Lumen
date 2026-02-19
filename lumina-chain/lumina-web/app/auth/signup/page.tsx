"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";

export default function SignupRedirect() {
  const router = useRouter();
  
  useEffect(() => {
    router.replace("/auth/login");
  }, [router]);

  return (
    <div className="login-shell">
      <div className="login-container">
        <p style={{ color: "var(--muted)" }}>Redirecting to wallet login...</p>
      </div>
    </div>
  );
}
