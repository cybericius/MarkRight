use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

fn main() {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    let private_b64 = B64.encode(signing_key.to_bytes());
    let public_b64 = B64.encode(verifying_key.to_bytes());

    println!("Ed25519 Keypair Generated");
    println!("========================");
    println!();
    println!("Private key (base64) — store as Worker secret:");
    println!("  {private_b64}");
    println!();
    println!("Public key (base64) — embed in license.rs:");
    println!("  {public_b64}");
}
