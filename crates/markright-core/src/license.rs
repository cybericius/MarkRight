use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use ed25519_dalek::{Signature, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Production public key for license verification (Ed25519, 32 bytes, base64-encoded).
const PUBLIC_KEY_B64: &str = "1Q8KGzZ76kk/wKEY5wLlqDt8xnqK4T6d/wG8Zi/nELE=";

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

/// Verify a license token string.
///
/// Token format: `base64(json_payload).base64(ed25519_signature)`
pub fn verify_license(token: &str) -> LicenseStatus {
    let Some((payload_b64, sig_b64)) = token.split_once('.') else {
        return LicenseStatus::default();
    };

    let Ok(payload_bytes) = B64.decode(payload_b64) else {
        return LicenseStatus::default();
    };

    let Ok(sig_bytes) = B64.decode(sig_b64) else {
        return LicenseStatus::default();
    };

    let Ok(pub_bytes) = B64.decode(PUBLIC_KEY_B64) else {
        return LicenseStatus::default();
    };

    let Ok(pub_array): Result<[u8; PUBLIC_KEY_LENGTH], _> = pub_bytes.try_into() else {
        return LicenseStatus::default();
    };

    let Ok(verifying_key) = VerifyingKey::from_bytes(&pub_array) else {
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

    /// Test secret key (base64). Must match the production public key above.
    const SECRET_KEY_B64: &str = "COAAMIwlYVdFHWAvfRTjXyaw+USngZV8rZXefTibf/A=";

    fn sign_token(payload_json: &str) -> String {
        let secret_bytes = B64.decode(SECRET_KEY_B64).unwrap();
        let secret_array: [u8; 32] = secret_bytes.try_into().unwrap();
        let signing_key = SigningKey::from_bytes(&secret_array);

        let payload_b64 = B64.encode(payload_json.as_bytes());
        let sig = signing_key.sign(payload_b64.as_bytes());
        let sig_b64 = B64.encode(sig.to_bytes());

        format!("{payload_b64}.{sig_b64}")
    }

    fn valid_payload() -> String {
        r#"{"email":"test@example.com","issued_at":"2026-01-01T00:00:00Z","tier":"pro"}"#
            .to_string()
    }

    #[test]
    fn valid_token() {
        let token = sign_token(&valid_payload());
        let status = verify_license(&token);
        assert!(status.valid);
        assert_eq!(status.email.as_deref(), Some("test@example.com"));
        assert_eq!(status.tier.as_deref(), Some("pro"));
    }

    #[test]
    fn tampered_payload() {
        let token = sign_token(&valid_payload());
        let (_, sig) = token.split_once('.').unwrap();
        // Replace payload with different content
        let tampered_payload = B64.encode(b"{\"email\":\"hacker@evil.com\",\"issued_at\":\"2026-01-01T00:00:00Z\",\"tier\":\"pro\"}");
        let tampered = format!("{tampered_payload}.{sig}");
        let status = verify_license(&tampered);
        assert!(!status.valid);
    }

    #[test]
    fn tampered_signature() {
        let token = sign_token(&valid_payload());
        let (payload, _) = token.split_once('.').unwrap();
        let bad_sig = B64.encode([0u8; SIGNATURE_LENGTH]);
        let tampered = format!("{payload}.{bad_sig}");
        let status = verify_license(&tampered);
        assert!(!status.valid);
    }

    #[test]
    fn bad_format_no_dot() {
        let status = verify_license("no-dot-here");
        assert!(!status.valid);
        assert!(status.email.is_none());
    }

    #[test]
    fn bad_format_not_base64() {
        let status = verify_license("!!!.!!!");
        assert!(!status.valid);
    }

    #[test]
    fn empty_token() {
        let status = verify_license("");
        assert!(!status.valid);
    }

    #[test]
    fn license_file_missing() {
        let dir = std::env::temp_dir().join("markright_test_no_license");
        let _ = std::fs::create_dir_all(&dir);
        let status = check_license_file(&dir);
        assert!(!status.valid);
    }

    #[test]
    fn license_file_valid() {
        let dir = tempfile::tempdir().unwrap();
        let token = sign_token(&valid_payload());
        std::fs::write(dir.path().join("license.key"), &token).unwrap();
        let status = check_license_file(dir.path());
        assert!(status.valid);
        assert_eq!(status.email.as_deref(), Some("test@example.com"));
    }

    /// End-to-end: sign with production key, verify with production public key.
    #[test]
    fn production_token_roundtrip() {
        let token = sign_token(
            r#"{"email":"akos@markright.app","issued_at":"2026-02-25T00:00:00Z","tier":"pro"}"#,
        );
        let status = verify_license(&token);
        assert!(status.valid, "Production-signed token must verify");
        assert_eq!(status.email.as_deref(), Some("akos@markright.app"));
        assert_eq!(status.tier.as_deref(), Some("pro"));
    }

    /// Verify an externally-generated token (from keygen sign_token binary).
    #[test]
    fn verify_externally_signed_token() {
        let token = "eyJlbWFpbCI6ImFrb3NAbWFya3JpZ2h0LmFwcCIsImlzc3VlZF9hdCI6IjIwMjYtMDMtMTBUMDA6MDA6MDBaIiwidGllciI6InBybyJ9.PjMB1ji8oPz0/xXk4keTuYWyaPAexHVa7q+ndFGlkpdrK8OAUY9vo/FeBWS2e9k1R8h8aSZ3pSF2VwjWqE1vAQ==";
        let status = verify_license(token);
        assert!(status.valid, "Externally signed token must verify");
        assert_eq!(status.email.as_deref(), Some("akos@markright.app"));
        assert_eq!(status.tier.as_deref(), Some("pro"));
    }
}
