import type { Env, LicensePayload } from "./types";
import { signLicenseToken } from "./crypto";
import { verifyPolarWebhook } from "./polar";
import { sendLicenseEmail } from "./resend";

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);

    if (request.method === "POST" && url.pathname === "/webhook/polar") {
      return handlePolarWebhook(request, env);
    }

    return new Response("Not found", { status: 404 });
  },
};

async function handlePolarWebhook(
  request: Request,
  env: Env,
): Promise<Response> {
  const body = await request.text();

  let payload;
  try {
    payload = verifyPolarWebhook(body, request.headers, env.POLAR_WEBHOOK_SECRET);
  } catch {
    return new Response("Invalid webhook signature", { status: 401 });
  }

  // Only process completed orders
  if (payload.type !== "order.created") {
    return new Response("Ignored event type", { status: 200 });
  }

  const email = payload.data.customer.email;
  const issuedAt = payload.data.created_at || new Date().toISOString();

  const licensePayload: LicensePayload = {
    email,
    issued_at: issuedAt,
    tier: "pro",
  };

  const token = await signLicenseToken(licensePayload, env.ED25519_PRIVATE_KEY);

  try {
    await sendLicenseEmail(email, token, env.RESEND_API_KEY);
  } catch (err) {
    console.error("Failed to send license email:", err);
    return new Response("Email delivery failed", { status: 500 });
  }

  return new Response("License issued", { status: 200 });
}
