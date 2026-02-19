export function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

export function hexToBytes(hex: string): Uint8Array {
  const h = hex.trim().toLowerCase().replace(/^0x/, "");
  if (h.length === 0) return new Uint8Array();
  if (h.length % 2 !== 0) {
    throw new Error("Invalid hex length");
  }
  const out = new Uint8Array(h.length / 2);
  for (let i = 0; i < out.length; i++) {
    out[i] = parseInt(h.slice(i * 2, i * 2 + 2), 16);
  }
  return out;
}

export function u8aToNumberArray(bytes: Uint8Array): number[] {
  return Array.from(bytes);
}

export function numberArrayToU8a(arr: number[]): Uint8Array {
  return new Uint8Array(arr.map((n) => n & 0xff));
}
