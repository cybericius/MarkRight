import type { PolarOrderPayload } from "./types";

/**
 * Verify a Polar.sh webhook using the Standard Webhooks spec (HMAC-SHA256).
 *
 * Polar's webhook secret is used as-is (full UTF-8 bytes) as the HMAC key.
 */
export async function verifyPolarWebhook(
  body: string,
  headers: Headers,
  secret: string,
): Promise<PolarOrderPayload> {
  const msgId = headers.get("webhook-id") ?? headers.get("svix-id") ?? "";
  const msgTimestamp = headers.get("webhook-timestamp") ?? headers.get("svix-timestamp") ?? "";
  const msgSignature = headers.get("webhook-signature") ?? headers.get("svix-signature") ?? "";

  if (!msgId || !msgTimestamp || !msgSignature) {
    throw new Error("Missing webhook signature headers");
  }

  const encoder = new TextEncoder();
  const secretBytes = encoder.encode(secret);

  const toSign = `${msgId}.${msgTimestamp}.${body}`;

  const key = await crypto.subtle.importKey(
    "raw",
    secretBytes,
    { name: "HMAC", hash: "SHA-256" },
    false,
    ["sign"],
  );

  const signature = await crypto.subtle.sign("HMAC", key, encoder.encode(toSign));
  const expectedSig = btoa(String.fromCharCode(...new Uint8Array(signature)));

  const signatures = msgSignature.split(" ");
  const verified = signatures.some((sig) => {
    const sigValue = sig.startsWith("v1,") ? sig.slice(3) : sig;
    return sigValue === expectedSig;
  });

  if (!verified) {
    throw new Error("Webhook signature mismatch");
  }

  return JSON.parse(body) as PolarOrderPayload;
}
