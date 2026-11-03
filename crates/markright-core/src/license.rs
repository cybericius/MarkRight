use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use ed25519_dalek::{Signature, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Production public key for license verification (Ed25519, 32 bytes, base64-encoded).
const PUBLIC_KEY_B64: &str = "Es1YykTZkMsOxXt92p73OomoMX1w0rGv1Pr+N1vR0t8=";

/// Result of verifying a license token.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LicenseStatus {
    pub valid: bool,
    pub email: Option<String>,
    pub tier: Option<String>,
}

/// Payload embedded in the license token.
#[derive(Debug, Deserialize)]
struct LicensePayload {
    email: String,
    #[allow(dead_code)]
    issued_at: String,
    tier: String,
}

/// Decode the built-in production public key.
fn production_key() -> Option<VerifyingKey> {
    let pub_bytes = B64.decode(PUBLIC_KEY_B64).ok()?;
    let pub_array: [u8; PUBLIC_KEY_LENGTH] = pub_bytes.try_into().ok()?;
    VerifyingKey::from_bytes(&pub_array).ok()
}

/// Verify a license token string against the production public key.
///
/// Token format: `base64(json_payload).base64(ed25519_signature)`
pub fn verify_license(token: &str) -> LicenseStatus {
    match production_key() {
        Some(key) => verify_license_with_key(token, &key),
        None => LicenseStatus::default(),
    }
}

/// Verify a license token string against the given public key.
fn verify_license_with_key(token: &str, verifying_key: &VerifyingKey) -> LicenseStatus {
    let Some((payload_b64, sig_b64)) = token.split_once('.') else {
        return LicenseStatus::default();
    };

    let Ok(payload_bytes) = B64.decode(payload_b64) else {
        return LicenseStatus::default();
    };

    let Ok(sig_bytes) = B64.decode(sig_b64) else {
        return LicenseStatus::default();
    };

    let Ok(sig_array): Result<[u8; SIGNATURE_LENGTH], _> = sig_bytes.try_into() else {
        return LicenseStatus::default();
    };

    let signature = Signature::from_bytes(&sig_array);

    // Verify signature over the raw base64 payload string (not decoded bytes)
    if verifying_key
        .verify_strict(payload_b64.as_bytes(), &signature)
        .is_err()
    {
        return LicenseStatus::default();
    }

    let Ok(payload) = serde_json::from_slice::<LicensePayload>(&payload_bytes) else {
        return LicenseStatus::default();
    };

    LicenseStatus {
        valid: true,
        email: Some(payload.email),
        tier: Some(payload.tier),
    }
}

/// Read and verify a license file from the given config directory.
///
/// Looks for `license.key` in `config_dir`.
pub fn check_license_file(config_dir: &Path) -> LicenseStatus {
    let path = config_dir.join("license.key");
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return LicenseStatus::default();
    };
    verify_license(contents.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    /// Throwaway key for tests only. It is NOT the production key, and the
    /// production signing key must never appear in this repository.
    fn test_key() -> SigningKey {
        SigningKey::from_bytes(&[7u8; 32])
    }

    fn sign_token(payload_json: &str) -> String {
        let payload_b64 = B64.encode(payload_json.as_bytes());
        let sig = test_key().sign(payload_b64.as_bytes());
        let sig_b64 = B64.encode(sig.to_bytes());
        format!("{payload_b64}.{sig_b64}")
    }

    fn verify(token: &str) -> LicenseStatus {
        verify_license_with_key(token, &test_key().verifying_key())
    }

    fn valid_payload() -> String {
        r#"{"email":"test@example.com","issued_at":"2026-01-01T00:00:00Z","tier":"pro"}"#
            .to_string()
    }

    #[test]
    fn production_key_decodes() {
        assert!(production_key().is_some());
    }

    #[test]
    fn test_key_is_not_production_key() {
        assert_ne!(Some(test_key().verifying_key()), production_key());
    }

    #[test]
    fn valid_token() {
        let status = verify(&sign_token(&valid_payload()));
        assert!(status.valid);
        assert_eq!(status.email.as_deref(), Some("test@example.com"));
        assert_eq!(status.tier.as_deref(), Some("pro"));
    }

    #[test]
    fn test_signed_token_rejected_by_production_key() {
        assert!(!verify_license(&sign_token(&valid_payload())).valid);
    }

    #[test]
    fn tampered_payload() {
        let token = sign_token(&valid_payload());
        let (_, sig) = token.split_once('.').unwrap();
        let tampered_payload = B64.encode(b"{\"email\":\"hacker@evil.com\",\"issued_at\":\"2026-01-01T00:00:00Z\",\"tier\":\"pro\"}");
        assert!(!verify(&format!("{tampered_payload}.{sig}")).valid);
    }

    #[test]
    fn tampered_signature() {
        let token = sign_token(&valid_payload());
        let (payload, _) = token.split_once('.').unwrap();
        let bad_sig = B64.encode([0u8; SIGNATURE_LENGTH]);
        assert!(!verify(&format!("{payload}.{bad_sig}")).valid);
    }

    #[test]
    fn bad_format_no_dot() {
        let status = verify_license("no-dot-here");
        assert!(!status.valid);
        assert!(status.email.is_none());
    }

    #[test]
    fn bad_format_not_base64() {
        assert!(!verify_license("!!!.!!!").valid);
    }

    #[test]
    fn empty_token() {
        assert!(!verify_license("").valid);
    }

    #[test]
    fn license_file_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!check_license_file(dir.path()).valid);
    }

    #[test]
    fn license_file_with_foreign_signature_rejected() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("license.key"), sign_token(&valid_payload())).unwrap();
        assert!(!check_license_file(dir.path()).valid);
    }
}
