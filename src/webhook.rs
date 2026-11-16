use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::{MelostoreError, Result};
use crate::models::bulk::BulkCallback;
use crate::models::transaction::{ActionCallback, RegularCallback, SmartCallback};

/// The header name used by Melo H2H to pass the callback signature.
pub const HEADER_SIGNATURE: &str = "X-H2H-Signature";

/// Computes HMAC-SHA256 according to RFC 2104 using `sha2::Sha256`.
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    const BLOCK_SIZE: usize = 64;
    let mut key_block = [0u8; BLOCK_SIZE];
    if key.len() > BLOCK_SIZE {
        let mut hasher = Sha256::new();
        hasher.update(key);
        let hashed_key = hasher.finalize();
        key_block[..32].copy_from_slice(&hashed_key);
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }

    let mut o_key_pad = [0x5cu8; BLOCK_SIZE];
    let mut i_key_pad = [0x36u8; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        o_key_pad[i] ^= key_block[i];
        i_key_pad[i] ^= key_block[i];
    }

    let mut inner_hasher = Sha256::new();
    inner_hasher.update(i_key_pad);
    inner_hasher.update(data);
    let inner_hash = inner_hasher.finalize();

    let mut outer_hasher = Sha256::new();
    outer_hasher.update(o_key_pad);
    outer_hasher.update(inner_hash);
    outer_hasher.finalize().into()
}

/// Constant-time comparison of two byte slices to mitigate timing side-channel attacks.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Polymorphic webhook payload sent to partner callback URL.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WebhookPayload {
    /// Action event (e.g. 2FA verification requested or updated).
    Action(ActionCallback),
    /// Child order completed within a bulk batch.
    Bulk(BulkCallback),
    /// Smart order status finalization.
    Smart(SmartCallback),
    /// Regular order status finalization.
    Regular(RegularCallback),
}

/// Verifies whether the incoming HTTP `X-H2H-Signature` header matches the HMAC-SHA256 of the exact raw body.
///
/// Uses constant-time comparison to mitigate timing side-channel attacks.
///
/// # Arguments
/// * `secret` - The Webhook Secret generated from the partner dashboard.
/// * `raw_body` - The unparsed, exact raw bytes of the incoming HTTP request body.
/// * `signature_hex` - The hexadecimal signature string provided in the `X-H2H-Signature` header.
pub fn verify_webhook_signature(secret: &str, raw_body: &[u8], signature_hex: &str) -> bool {
    if secret.trim().is_empty() || signature_hex.trim().is_empty() {
        return false;
    }

    let expected = hmac_sha256(secret.trim().as_bytes(), raw_body);

    let Ok(provided) = hex::decode(signature_hex.trim()) else {
        return false;
    };

    constant_time_eq(&expected, &provided)
}

/// Parses the raw webhook request body into a typed [`WebhookPayload`].
pub fn parse_webhook(raw_body: &[u8]) -> Result<WebhookPayload> {
    serde_json::from_slice::<WebhookPayload>(raw_body).map_err(MelostoreError::Json)
}

/// Validates the HMAC-SHA256 signature and parses the webhook request body in one step.
///
/// Returns `Err(MelostoreError::WebhookVerificationFailed)` if the signature does not match.
pub fn parse_and_verify_webhook(
    secret: &str,
    raw_body: &[u8],
    signature_hex: &str,
) -> Result<WebhookPayload> {
    if !verify_webhook_signature(secret, raw_body, signature_hex) {
        return Err(MelostoreError::WebhookVerificationFailed(
            "HMAC-SHA256 signature mismatch or invalid format".into(),
        ));
    }

    parse_webhook(raw_body)
}
