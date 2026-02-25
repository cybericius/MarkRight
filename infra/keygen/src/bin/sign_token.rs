use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use ed25519_dalek::{Signer, SigningKey};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let email = args.get(1).map_or("test@example.com", |s| s.as_str());

    // Load private key from env or use the production key
    let private_key_b64 = std::env::var("ED25519_PRIVATE_KEY")
        .unwrap_or_else(|_| {
            eprintln!("Reading private key from .claude/secrets.env ...");
            let secrets = std::fs::read_to_string("../../.claude/secrets.env")
                .expect("Could not read .claude/secrets.env — run from infra/keygen/");
            secrets
                .lines()
                .find(|l| l.starts_with("ED25519_PRIVATE_KEY="))
                .expect("ED25519_PRIVATE_KEY not found in secrets.env")
                .strip_prefix("ED25519_PRIVATE_KEY=")
                .unwrap()
                .to_string()
        });

    let secret_bytes = B64.decode(&private_key_b64).expect("Invalid base64 private key");
    let secret_array: [u8; 32] = secret_bytes.try_into().expect("Key must be 32 bytes");
    let signing_key = SigningKey::from_bytes(&secret_array);

    let issued_at = chrono_now();
    let payload_json = format!(
        r#"{{"email":"{email}","issued_at":"{issued_at}","tier":"pro"}}"#
    );

    let payload_b64 = B64.encode(payload_json.as_bytes());
    let sig = signing_key.sign(payload_b64.as_bytes());
    let sig_b64 = B64.encode(sig.to_bytes());

    let token = format!("{payload_b64}.{sig_b64}");

    eprintln!("Payload: {payload_json}");
    eprintln!("Email:   {email}");
    eprintln!("Tier:    pro");
    eprintln!("Issued:  {issued_at}");
    eprintln!();

    // Print just the token to stdout (for piping)
    println!("{token}");
}

fn chrono_now() -> String {
    // Simple UTC timestamp without pulling in chrono
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Rough ISO 8601 — good enough for license tokens
    let days = secs / 86400;
    let years = 1970 + days / 365;
    let remaining_days = days % 365;
    let months = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    format!("{years}-{months:02}-{day:02}T00:00:00Z")
}
