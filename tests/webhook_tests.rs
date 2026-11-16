use melostore_sdk_rust::webhook::{
    constant_time_eq, hmac_sha256, parse_and_verify_webhook, parse_webhook,
    verify_webhook_signature, WebhookPayload,
};

#[test]
fn test_rfc2104_hmac_sha256_known_vector() {
    // RFC 4231 Test Case 2 (HMAC-SHA256):
    // Key: "Jefe"
    // Data: "what do ya want for nothing?"
    // Digest: 5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843
    let key = b"Jefe";
    let data = b"what do ya want for nothing?";
    let expected_hex = "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843";

    let result = hmac_sha256(key, data);
    assert_eq!(hex::encode(result), expected_hex);
}

#[test]
fn test_constant_time_eq() {
    let a = b"secret_hash_value_1234567890abcdef";
    let b = b"secret_hash_value_1234567890abcdef";
    let c = b"secret_hash_value_1234567890abcdeg";
    let d = b"short";

    assert!(constant_time_eq(a, b));
    assert!(!constant_time_eq(a, c));
    assert!(!constant_time_eq(a, d));
}

#[test]
fn test_webhook_signature_verification_success() {
    let secret = "whsec_test_secret_key_123";
    let raw_body = br#"{"id":"11111111-1111-4111-8111-111111111111","status":"success"}"#;

    let signature = hex::encode(hmac_sha256(secret.as_bytes(), raw_body));

    assert!(verify_webhook_signature(secret, raw_body, &signature));
}

#[test]
fn test_webhook_signature_verification_failure_tampered_body() {
    let secret = "whsec_test_secret_key_123";
    let raw_body = br#"{"id":"11111111-1111-4111-8111-111111111111","status":"success"}"#;
    let tampered_body = br#"{"id":"11111111-1111-4111-8111-111111111111","status":"failed"}"#;

    let signature = hex::encode(hmac_sha256(secret.as_bytes(), raw_body));

    assert!(!verify_webhook_signature(secret, tampered_body, &signature));
}

#[test]
fn test_webhook_signature_verification_failure_wrong_secret() {
    let secret = "whsec_test_secret_key_123";
    let wrong_secret = "whsec_wrong_secret_456";
    let raw_body = br#"{"id":"11111111-1111-4111-8111-111111111111","status":"success"}"#;

    let signature = hex::encode(hmac_sha256(secret.as_bytes(), raw_body));

    assert!(!verify_webhook_signature(wrong_secret, raw_body, &signature));
}

#[test]
fn test_parse_regular_callback_success() {
    let json_data = br#"{
        "id": "11111111-1111-4111-8111-111111111111",
        "buyer_trx_id": "example-order-001",
        "sku_code": "EXAMPLE-10",
        "customer_target": "123456789",
        "customer_target_zone": "1234",
        "status": "success",
        "serial_number": "EXAMPLE-SN-001",
        "is_sandbox": true,
        "created_at": "2026-10-02T03:00:00+00:00",
        "price_charged": 1000.0,
        "updated_at": "2026-10-02T03:00:00+00:00"
    }"#;

    let parsed = parse_webhook(json_data).expect("Should parse regular callback");
    match parsed {
        WebhookPayload::Regular(cb) => {
            assert_eq!(cb.id, "11111111-1111-4111-8111-111111111111");
            assert_eq!(cb.buyer_trx_id, "example-order-001");
            assert_eq!(cb.status, "success");
            assert_eq!(cb.serial_number.as_deref(), Some("EXAMPLE-SN-001"));
            assert!(cb.is_sandbox);
            assert_eq!(cb.price_charged, 1000.0);
        }
        _ => panic!("Expected Regular callback"),
    }
}

#[test]
fn test_parse_smart_callback() {
    let json_data = br#"{
        "id": "22222222-2222-4222-8222-222222222222",
        "buyer_trx_id": "smart-order-001",
        "sku_code": "SMART-MLBB",
        "customer_target": "123456789",
        "customer_target_zone": "1234",
        "status": "success",
        "serial_number": "SN-SMART-999",
        "is_sandbox": false,
        "created_at": "2026-10-02T03:00:00+00:00",
        "price_charged": 15000.0,
        "updated_at": "2026-10-02T03:00:05+00:00",
        "max_bid": 16000,
        "fulfilled_sku_code": "SUPPLIER-MLBB-86",
        "attempts_count": 2
    }"#;

    let parsed = parse_webhook(json_data).expect("Should parse smart callback");
    match parsed {
        WebhookPayload::Smart(cb) => {
            assert_eq!(cb.buyer_trx_id, "smart-order-001");
            assert_eq!(cb.status, "success");
            assert_eq!(cb.max_bid, Some(16000));
            assert_eq!(cb.fulfilled_sku_code.as_deref(), Some("SUPPLIER-MLBB-86"));
            assert_eq!(cb.attempts_count, 2);
        }
        _ => panic!("Expected Smart callback"),
    }
}

#[test]
fn test_parse_bulk_callback() {
    let json_data = br#"{
        "id": "33333333-3333-4333-8333-333333333333",
        "buyer_trx_id": "bulk-child-001",
        "sku_code": "EXAMPLE-10",
        "customer_target": "123456789",
        "customer_target_zone": "1234",
        "status": "success",
        "serial_number": "BULK-SN-001",
        "is_sandbox": true,
        "created_at": "2026-10-02T03:00:00+00:00",
        "price_charged": 1000.0,
        "updated_at": "2026-10-02T03:00:00+00:00",
        "bulk_order_id": "bulk-order-uuid-999",
        "buyer_bulk_id": "partner-batch-100",
        "bulk_sequence": 1
    }"#;

    let parsed = parse_webhook(json_data).expect("Should parse bulk callback");
    match parsed {
        WebhookPayload::Bulk(cb) => {
            assert_eq!(cb.bulk_order_id, "bulk-order-uuid-999");
            assert_eq!(cb.buyer_bulk_id, "partner-batch-100");
            assert_eq!(cb.bulk_sequence, 1);
            assert_eq!(cb.status, "success");
        }
        _ => panic!("Expected Bulk callback"),
    }
}

#[test]
fn test_parse_action_callback() {
    let json_data = br#"{
        "event": "transaction.action_change",
        "event_id": "22222222-2222-4222-8222-000000000004",
        "id": "example-transaction-001",
        "buyer_trx_id": "example-order-001",
        "status": "processing",
        "updated_at": "2026-10-10T08:04:00+07:00",
        "need_action": true,
        "action": {
            "id": "11111111-1111-4111-8111-111111111111",
            "type": "two_factor",
            "state": "required",
            "version": 4,
            "message": "Two-factor authentication is required to continue this order.",
            "can_submit": true,
            "updated_at": "2026-10-10T08:04:00+07:00"
        }
    }"#;

    let parsed = parse_webhook(json_data).expect("Should parse action callback");
    match parsed {
        WebhookPayload::Action(cb) => {
            assert_eq!(cb.event, "transaction.action_change");
            assert_eq!(cb.event_id, "22222222-2222-4222-8222-000000000004");
            assert!(cb.need_action);
            let action = cb.action.expect("Should have action");
            assert_eq!(action.action_type, "two_factor");
            assert_eq!(action.state, "required");
            assert_eq!(action.version, 4);
            assert!(action.can_submit);
        }
        _ => panic!("Expected Action callback"),
    }
}

#[test]
fn test_parse_and_verify_webhook_e2e() {
    let secret = "whsec_super_secret_987654";
    let body = br#"{
        "id": "11111111-1111-4111-8111-111111111111",
        "buyer_trx_id": "order-101",
        "sku_code": "PROD-1",
        "customer_target": "user123",
        "customer_target_zone": null,
        "status": "success",
        "serial_number": "SN-001",
        "is_sandbox": false,
        "created_at": "2026-10-02T03:00:00+00:00",
        "price_charged": 50000.0,
        "updated_at": "2026-10-02T03:00:10+00:00"
    }"#;

    let signature = hex::encode(hmac_sha256(secret.as_bytes(), body));

    let event = parse_and_verify_webhook(secret, body, &signature).expect("Verification must pass");
    match event {
        WebhookPayload::Regular(cb) => {
            assert_eq!(cb.buyer_trx_id, "order-101");
            assert_eq!(cb.price_charged, 50000.0);
        }
        _ => panic!("Expected Regular callback"),
    }
}
