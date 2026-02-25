import * as ed from "@noble/ed25519";
import { sha512 } from "@noble/hashes/sha512";
import type { LicensePayload } from "./types";

// ed25519 requires sha512 configuration
ed.etc.sha512Sync = (...m: Uint8Array[]) => {
  const h = sha512.create();
  for (const msg of m) h.update(msg);
  return h.digest();
};

/**
 * Generate a license token compatible with the Rust verify_license() function.
 *
 * Token format: `base64(json_payload).base64(ed25519_signature)`
 *
 * The signature is computed over the base64-encoded payload string bytes,
 * matching `verify_strict(payload_b64.as_bytes(), &signature)` in license.rs.
 */
export async function signLicenseToken(
  payload: LicensePayload,
  privateKeyB64: string,
): Promise<string> {
  const privateKeyBytes = base64ToBytes(privateKeyB64);

  const payloadJson = JSON.stringify(payload);
  const payloadB64 = bytesToBase64(new TextEncoder().encode(payloadJson));

  // Sign over the base64 string bytes (matching Rust verifier)
  const messageBytes = new TextEncoder().encode(payloadB64);
  const signature = await ed.signAsync(messageBytes, privateKeyBytes);

  const sigB64 = bytesToBase64(signature);
  return `${payloadB64}.${sigB64}`;
}

function base64ToBytes(b64: string): Uint8Array {
  const binary = atob(b64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  return bytes;
}

function bytesToBase64(bytes: Uint8Array): string {
  let binary = "";
  for (const b of bytes) {
    binary += String.fromCharCode(b);
  }
  return btoa(binary);
}
