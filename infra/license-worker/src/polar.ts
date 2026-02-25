import { Webhook } from "svix";
import type { PolarOrderPayload } from "./types";

/**
 * Verify a Polar.sh webhook signature and parse the payload.
 *
 * Polar uses Svix for webhook delivery; signature is verified
 * using the svix-id, svix-timestamp, and svix-signature headers.
 */
export function verifyPolarWebhook(
  body: string,
  headers: Headers,
  secret: string,
): PolarOrderPayload {
  const wh = new Webhook(secret);

  const svixId = headers.get("svix-id") ?? "";
  const svixTimestamp = headers.get("svix-timestamp") ?? "";
  const svixSignature = headers.get("svix-signature") ?? "";

  const payload = wh.verify(body, {
    "svix-id": svixId,
    "svix-timestamp": svixTimestamp,
    "svix-signature": svixSignature,
  }) as PolarOrderPayload;

  return payload;
}
