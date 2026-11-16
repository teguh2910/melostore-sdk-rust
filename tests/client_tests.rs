use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use melostore_sdk_rust::models::account::{CheckNicknameRequest, CheckPurchaseLimitRequest};
use melostore_sdk_rust::models::bulk::CreateBulkOrderRequest;
use melostore_sdk_rust::models::transaction::{
    CreateSmartTransactionRequest, CreateTransactionRequest, SubmitTwoFactorRequest,
    TransactionStatusData,
};
use melostore_sdk_rust::{MelostoreClient, MelostoreConfig, MelostoreError};

async fn create_test_client(server: &MockServer) -> MelostoreClient {
    let config = MelostoreConfig::builder()
        .api_key("test_api_key_12345")
        .secret_key("test_secret_key_67890")
        .base_url(server.uri())
        .build()
        .expect("Config build failed");

    MelostoreClient::with_config(config).expect("Client init failed")
}

#[tokio::test]
async fn test_auth_headers_injected() {
    let server = MockServer::start().await;
    let client = create_test_client(&server).await;

    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/profile/balance"))
        .and(header("x-api-key", "test_api_key_12345"))
        .and(header("x-secret-key", "test_secret_key_67890"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "data": {
                "h2h_balance": 500000.0,
                "is_sandbox_mode": false,
                "usd_idr_rate": 16500.0,
                "display_usd_idr_rate": 16500.0,
                "display_rate_updated_at": "2026-10-02T03:00:00+00:00",
                "h2h_balance_usd": 30.3
            }
        })))
        .mount(&server)
        .await;

    let balance = client.get_balance().await.expect("Failed to get balance");
    assert_eq!(balance.h2h_balance, 500000.0);
    assert!(!balance.is_sandbox_mode);
}

#[tokio::test]
async fn test_get_profile() {
    let server = MockServer::start().await;
    let client = create_test_client(&server).await;

    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/profile"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "data": {
                "id": "11111111-1111-4111-8111-111111111111",
                "name": "Partner Store",
                "email": "partner@store.com",
                "tier": {
                    "name": "VIP",
                    "code": "vip",
                    "rate_limit": 120,
                    "max_ips": 5
                },
                "h2h_balance": 1500000.0,
                "is_sandbox_mode": false,
                "total_volume_30_days": 10000000.0,
                "webhook_url": "https://partner.com/webhook",
                "created_at": "2026-01-01T00:00:00+00:00",
                "usd_idr_rate": 16000.0,
                "display_usd_idr_rate": 16000.0,
                "display_rate_updated_at": "2026-01-01T00:00:00+00:00"
            }
        })))
        .mount(&server)
        .await;

    let profile = client.get_profile().await.expect("Failed to get profile");
    assert_eq!(profile.name, "Partner Store");
    assert_eq!(profile.tier.code, "vip");
    assert_eq!(profile.tier.rate_limit, 120);
}

#[tokio::test]
async fn test_catalog_endpoints() {
    let server = MockServer::start().await;
    let client = create_test_client(&server).await;

    // 1. Pricelist
    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/pricelists"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "data": [
                {
                    "name": "Mobile Legends 86 Diamonds",
                    "sku_code": "MLBB-86",
                    "price": 19500.0,
                    "category_name": "Mobile Legends",
                    "brand_id": 1,
                    "type_name": "Direct Topup",
                    "status": "active"
                }
            ],
            "meta": {
                "brands": [
                    { "id": 1, "name": "Mobile Legends", "thumbnail": null, "order": 1 }
                ],
                "inquiry_forms": []
            }
        })))
        .mount(&server)
        .await;

    let pricelist = client.get_pricelist(None).await.expect("Failed to get pricelist");
    assert_eq!(pricelist.data.len(), 1);
    assert_eq!(pricelist.data[0].sku_code, "MLBB-86");

    // 2. Pricelist by Brand
    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/pricelists/mobile-legends"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "data": [
                {
                    "name": "Mobile Legends 172 Diamonds",
                    "sku_code": "MLBB-172",
                    "price": 38000.0,
                    "category_name": "Mobile Legends",
                    "status": "active"
                }
            ],
            "meta": {
                "brands": [],
                "inquiry_forms": {}
            },
            "category": {
                "name": "Mobile Legends",
                "slug": "mobile-legends"
            }
        })))
        .mount(&server)
        .await;

    let brand_pricelist = client
        .get_pricelist_by_brand("mobile-legends", None)
        .await
        .expect("Failed to get brand pricelist");
    assert_eq!(brand_pricelist.data[0].sku_code, "MLBB-172");

    // 3. Brands directory
    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/pricelists/categories"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "data": [
                {
                    "brand_id": 1,
                    "name": "Mobile Legends",
                    "slug": "mobile-legends"
                }
            ]
        })))
        .mount(&server)
        .await;

    let brands = client.get_catalogue_brands().await.expect("Failed to get brands");
    assert_eq!(brands.len(), 1);
    assert_eq!(brands[0].slug, "mobile-legends");
}

#[tokio::test]
async fn test_smart_catalog_endpoints() {
    let server = MockServer::start().await;
    let client = create_test_client(&server).await;

    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/smart-pricelists"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "data": [
                {
                    "name": "Weekly Diamond Pass",
                    "category_name": "Mobile Legends",
                    "status": "active",
                    "smart_sku_code": "SMART-WDP",
                    "min_price": 27000.0,
                    "max_price": 28500.0,
                    "variant_count": 3,
                    "variants": [
                        { "sku_code": "SUPPLIER-A-WDP", "price": 27000.0, "status": "active" },
                        { "sku_code": "SUPPLIER-B-WDP", "price": 27500.0, "status": "active" }
                    ]
                }
            ],
            "meta": {
                "brands": [],
                "inquiry_forms": {}
            }
        })))
        .mount(&server)
        .await;

    let smart = client.get_smart_pricelist(None).await.expect("Failed to get smart pricelist");
    assert_eq!(smart.data.len(), 1);
    assert_eq!(smart.data[0].smart_sku_code, "SMART-WDP");
    assert_eq!(smart.data[0].variants.len(), 2);
}

#[tokio::test]
async fn test_create_and_get_transaction() {
    let server = MockServer::start().await;
    let client = create_test_client(&server).await;

    // Create Transaction
    Mock::given(method("POST"))
        .and(path("/api/v1/h2h/transaction"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "data": {
                "id": "trx-uuid-111",
                "buyer_trx_id": "test-order-001",
                "sku_code": "MLBB-86",
                "customer_target": "12345678",
                "customer_target_zone": "1234",
                "price": 19500.0,
                "status": "pending",
                "serial_number": null,
                "webhook_delivery_status": "pending",
                "is_sandbox": false,
                "created_at": "2026-10-02T03:00:00+00:00"
            }
        })))
        .mount(&server)
        .await;

    let req = CreateTransactionRequest::new("MLBB-86", "12345678", "test-order-001");
    let trx = client.create_transaction(req).await.expect("Create transaction failed");
    assert_eq!(trx.id, "trx-uuid-111");
    assert_eq!(trx.status, "pending");

    // Get Transaction Status
    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/transaction/test-order-001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "data": {
                "id": "trx-uuid-111",
                "buyer_trx_id": "test-order-001",
                "sku_code": "MLBB-86",
                "customer_target": "12345678",
                "customer_target_zone": "1234",
                "price": 19500.0,
                "status": "success",
                "serial_number": "SN-DELIVERED-001",
                "webhook_delivery_status": "delivered",
                "is_sandbox": false,
                "created_at": "2026-10-02T03:00:00+00:00"
            }
        })))
        .mount(&server)
        .await;

    let status = client
        .get_transaction("test-order-001")
        .await
        .expect("Get transaction status failed");
    match status {
        TransactionStatusData::Regular(r) => {
            assert_eq!(r.status, "success");
            assert_eq!(r.serial_number.as_deref(), Some("SN-DELIVERED-001"));
        }
        TransactionStatusData::Smart(_) => panic!("Expected regular transaction"),
    }
}

#[tokio::test]
async fn test_create_smart_transaction() {
    let server = MockServer::start().await;
    let client = create_test_client(&server).await;

    Mock::given(method("POST"))
        .and(path("/api/v1/h2h/smart-transaction"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "data": {
                "id": "smart-trx-uuid-222",
                "buyer_trx_id": "smart-order-101",
                "sku_code": "SMART-WDP",
                "customer_target": "98765432",
                "price": 27000.0,
                "status": "processing",
                "webhook_delivery_status": "pending",
                "is_sandbox": false,
                "created_at": "2026-10-02T03:00:00+00:00",
                "max_bid": 28000
            }
        })))
        .mount(&server)
        .await;

    let req = CreateSmartTransactionRequest::new("SMART-WDP", "98765432", "smart-order-101", 28000);
    let ack = client.create_smart_transaction(req).await.expect("Smart transaction failed");
    assert_eq!(ack.id, "smart-trx-uuid-222");
    assert_eq!(ack.max_bid, Some(28000));
}

#[tokio::test]
async fn test_two_factor_auth() {
    let server = MockServer::start().await;
    let client = create_test_client(&server).await;

    Mock::given(method("POST"))
        .and(path("/api/v1/h2h/transactions/2fa"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "message": "2FA code accepted",
            "data": {
                "id": "action-uuid-333",
                "type": "two_factor",
                "state": "submitted",
                "version": 2,
                "message": "Code submitted successfully",
                "can_submit": false,
                "updated_at": "2026-10-02T03:01:00+00:00"
            }
        })))
        .mount(&server)
        .await;

    let req = SubmitTwoFactorRequest::new("order-2fa-101", "123456");
    let action = client.submit_two_factor(req).await.expect("2FA submission failed");
    assert_eq!(action.state, "submitted");
    assert!(!action.can_submit);
}

#[tokio::test]
async fn test_bulk_orders_and_invoice() {
    let server = MockServer::start().await;
    let client = create_test_client(&server).await;

    // Create Bulk
    Mock::given(method("POST"))
        .and(path("/api/v1/h2h/bulk-orders"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "success": true,
            "message": "Bulk order created",
            "data": {
                "id": "bulk-uuid-444",
                "buyer_bulk_id": "batch-2026-001",
                "sku_code": "MLBB-86",
                "product_name": "Mobile Legends 86 Diamonds",
                "customer_target": "12345678",
                "order_count": 5,
                "unit_price": 19500.0,
                "total_debit": 97500.0,
                "status": "processing",
                "is_sandbox": false,
                "invoice_available": true,
                "created_at": "2026-10-02T03:00:00+00:00",
                "total_count": 5,
                "success_count": 0,
                "failed_count": 0,
                "processed_count": 0,
                "remaining_count": 5,
                "processing_count": 1,
                "queued_count": 4
            }
        })))
        .mount(&server)
        .await;

    let req = CreateBulkOrderRequest::new("MLBB-86", "12345678", "batch-2026-001", 5);
    let bulk = client.create_bulk_order(req).await.expect("Create bulk failed");
    assert_eq!(bulk.id, "bulk-uuid-444");
    assert_eq!(bulk.total_debit, 97500.0);

    // Get Bulk Detail
    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/bulk-orders/bulk-uuid-444"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "data": {
                "id": "bulk-uuid-444",
                "buyer_bulk_id": "batch-2026-001",
                "sku_code": "MLBB-86",
                "product_name": "Mobile Legends 86 Diamonds",
                "customer_target": "12345678",
                "order_count": 5,
                "unit_price": 19500.0,
                "total_debit": 97500.0,
                "status": "completed",
                "is_sandbox": false,
                "invoice_available": true,
                "created_at": "2026-10-02T03:00:00+00:00",
                "total_count": 5,
                "success_count": 5,
                "failed_count": 0,
                "processed_count": 5,
                "remaining_count": 0,
                "processing_count": 0,
                "queued_count": 0
            }
        })))
        .mount(&server)
        .await;

    let detail = client.get_bulk_order("bulk-uuid-444").await.expect("Get bulk detail failed");
    assert_eq!(detail.status, "completed");
    assert_eq!(detail.success_count, 5);

    // Download PDF Invoice
    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/bulk-orders/bulk-uuid-444/invoice"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/pdf")
                .set_body_bytes(b"%PDF-1.4 Mock PDF Content".to_vec()),
        )
        .mount(&server)
        .await;

    let pdf = client
        .get_bulk_invoice_pdf("bulk-uuid-444")
        .await
        .expect("Invoice download failed");
    assert_eq!(pdf, b"%PDF-1.4 Mock PDF Content");
}

#[tokio::test]
async fn test_account_inquiries() {
    let server = MockServer::start().await;
    let client = create_test_client(&server).await;

    // Check Nickname
    Mock::given(method("POST"))
        .and(path("/api/v1/h2h/check-nickname"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "data": {
                "game_code": "mobile_legends",
                "customer_target": "12345678",
                "customer_target_zone": "1234",
                "username": "ProGamerID",
                "region": "ID",
                "region_type": "national"
            },
            "billing": {
                "charged_mc": 0.0,
                "free_remaining_today": 99,
                "transaction_id": null
            }
        })))
        .mount(&server)
        .await;

    let req = CheckNicknameRequest::new("mobile_legends", "12345678").with_zone("1234");
    let result = client.check_nickname(req).await.expect("Check nickname failed");
    assert_eq!(result.data.username, "ProGamerID");
    assert_eq!(result.billing.free_remaining_today, Some(99));

    // Check MLBB Purchase Limit
    Mock::given(method("POST"))
        .and(path("/api/v1/h2h/mobile-legends/purchase-limit"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "data": {
                "customer_target": "12345678",
                "customer_target_zone": "1234",
                "username": "ProGamerID",
                "weekly_pass": {
                    "limit_reached": false,
                    "items": [
                        { "package_code": "WDP", "limit_reached": false }
                    ]
                }
            }
        })))
        .mount(&server)
        .await;

    let limit_req = CheckPurchaseLimitRequest::new("12345678", "1234");
    let limit = client
        .check_mlbb_purchase_limit(limit_req)
        .await
        .expect("Check limit failed");
    assert_eq!(limit.data.username, "ProGamerID");
    let pass = limit.data.weekly_pass.expect("Expected weekly pass");
    assert!(!pass.limit_reached);
}

#[tokio::test]
async fn test_error_handling() {
    let server = MockServer::start().await;
    let client = create_test_client(&server).await;

    // 1. 401 Unauthorized
    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/profile"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "success": false,
            "message": "Unauthorized: Missing X-API-Key or X-Secret-Key headers."
        })))
        .mount(&server)
        .await;

    let err = client.get_profile().await.unwrap_err();
    match err {
        MelostoreError::Api { status_code, message, .. } => {
            assert_eq!(status_code, 401);
            assert!(message.contains("Unauthorized"));
        }
        _ => panic!("Expected Api error"),
    }

    // 2. 429 Rate Limited with Retry-After header
    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/profile/balance"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "15")
                .set_body_json(serde_json::json!({
                    "success": false,
                    "message": "Too Many Requests"
                })),
        )
        .mount(&server)
        .await;

    let err = client.get_balance().await.unwrap_err();
    match err {
        MelostoreError::RateLimited { retry_after, message } => {
            assert_eq!(retry_after, Some(15));
            assert_eq!(message, "Too Many Requests");
        }
        _ => panic!("Expected RateLimited error"),
    }
}
