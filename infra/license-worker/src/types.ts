export interface Env {
  ED25519_PRIVATE_KEY: string;
  POLAR_WEBHOOK_SECRET: string;
  RESEND_API_KEY: string;
}

export interface PolarOrderPayload {
  type: string;
  data: {
    id: string;
    customer: {
      email: string;
      name?: string;
    };
    product: {
      name: string;
    };
    created_at: string;
  };
}

export interface LicensePayload {
  email: string;
  issued_at: string;
  tier: string;
}
