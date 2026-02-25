# License Pipeline Setup

End-to-end flow: Polar.sh purchase → Cloudflare Worker → Ed25519 signed license → email via Resend.

## Architecture

```
Customer buys on Polar.sh
  → Polar fires order.created webhook
  → Cloudflare Worker receives & verifies (Svix)
  → Worker signs license token (Ed25519)
  → Worker sends license key email (Resend API)
  → Customer pastes key in MarkRight Settings → activated
```

## Services

| Service | Purpose | URL |
|---------|---------|-----|
| Polar.sh | Payment & storefront | https://dashboard.polar.sh |
| Cloudflare Workers | Webhook handler + token signing | https://dash.cloudflare.com |
| Resend | Transactional email | https://resend.com |

## Setup Steps

### 1. Polar.sh

1. Create an organization (or use personal account)
2. Go to **Products** → **Catalogue** → create product:
   - Name: `MarkRight Pro License`
   - Pricing: Pay what you want (minimum $5)
   - Type: Digital product / one-time payment
3. Go to **Settings** → **Webhooks** → **Add Webhook**:
   - URL: `https://markright-license.akos-zalavary.workers.dev/webhook/polar`
   - Format: **raw** (required for Svix signature verification)
   - Events: `order.created`
4. Copy the **Signing Secret** (starts with `polar_whs_...`)
5. Storefront URL: check dashboard sidebar for "Storefront" link

### 2. Resend

1. Sign up at https://resend.com (free tier: 100 emails/day)
2. **Domains** → **Add Domain** → add DNS records (TXT, MX, DKIM) → verify
3. **API Keys** → **Create API Key** → copy key (starts with `re_...`)
4. Emails sent from: `MarkRight <license@complitask.com>` (using complitask.com domain)

### 3. Cloudflare Worker

1. Install wrangler: `pnpm add -g wrangler`
2. Login: `wrangler login`
3. Register a workers.dev subdomain if first time (Cloudflare dashboard → Workers → Onboarding)
4. From `infra/license-worker/`:
   ```sh
   pnpm install
   wrangler deploy
   ```
5. Set secrets:
   ```sh
   echo "YOUR_KEY" | wrangler secret put ED25519_PRIVATE_KEY
   echo "YOUR_SECRET" | wrangler secret put POLAR_WEBHOOK_SECRET
   echo "YOUR_KEY" | wrangler secret put RESEND_API_KEY
   ```

### 4. Ed25519 Keypair

Generated via `infra/keygen/`. Public key is embedded in `crates/markright-core/src/license.rs`.

To manually sign a test token:
```sh
cd infra/keygen
ED25519_PRIVATE_KEY=... cargo run --bin sign_token -- test@example.com
```

## Worker URL

`https://markright-license.akos-zalavary.workers.dev/webhook/polar`

## Secrets Location

Local backup: `.claude/secrets.env` (gitignored)

## Testing

1. Tail worker logs: `wrangler tail --format pretty` (from `infra/license-worker/`)
2. Buy your own product on Polar.sh storefront to trigger the full flow
3. Check Polar dashboard → Webhooks → Deliveries for webhook status
4. Check email for license key delivery

## Troubleshooting

- **401 Invalid webhook signature**: Check `POLAR_WEBHOOK_SECRET` matches Polar dashboard
- **Resend errors**: Verify domain is confirmed, API key is valid
- **No webhook delivery**: Ensure product is published and webhook has `order.created` event selected
