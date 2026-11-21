use melostore_server::{config::Config, routes, AppState};
use reqwest::StatusCode;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn spawn_test_server(mock_base_url: &str) -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let cfg = Config {
        bind_addr: "127.0.0.1:0".to_string(),
        api_key: "test_api_key_12345".to_string(),
        secret_key: "test_secret_key_67890".to_string(),
        webhook_secret: Some("test_secret".to_string()),
        base_url: Some(mock_base_url.to_string()),
        sandbox_mode: true,
        cors_origin: "*".to_string(),
    };

    let state = AppState::new(cfg).unwrap();
    let app = routes::router(state);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (addr, handle)
}

#[tokio::test]
async fn test_health_check() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/profile/balance"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "h2h_balance": 500000.0,
                "is_sandbox_mode": true,
                "usd_idr_rate": 16500.0,
                "display_usd_idr_rate": 16500.0,
                "display_rate_updated_at": "2026-10-02T03:00:00+00:00",
                "h2h_balance_usd": 30.3
            }
        })))
        .mount(&mock)
        .await;

    let (addr, _handle) = spawn_test_server(&mock.uri()).await;
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("http://{addr}/api/v1/health"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["provider"]["connected"], true);
    assert_eq!(body["provider"]["balance"], 500000.0);
}

#[tokio::test]
async fn test_get_products() {
    let mock = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/pricelists"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": [
                {
                    "id": 1,
                    "uuid": "prod-1",
                    "name": "50 Diamonds",
                    "sku_code": "MLBB-50",
                    "price": 14000.0,
                    "category_name": "Game",
                    "status": "active"
                }
            ],
            "meta": {
                "brands": [{"id": 1, "name": "Mobile Legends", "thumbnail": null, "order": 1}],
                "inquiry_forms": []
            }
        })))
        .mount(&mock)
        .await;

    let (addr, _handle) = spawn_test_server(&mock.uri()).await;
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("http://{addr}/api/v1/products"))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["data"][0]["sku_code"], "MLBB-50");
}

#[tokio::test]
async fn test_validate_account() {
    let mock = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/h2h/check-nickname"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "game_code": "mlbb",
                "customer_target": "12345678",
                "customer_target_zone": "1234",
                "username": "Gamer123"
            },
            "billing": {
                "charged_mc": 0.0,
                "free_remaining_today": 99,
                "transaction_id": null
            }
        })))
        .mount(&mock)
        .await;

    let (addr, _handle) = spawn_test_server(&mock.uri()).await;
    let client = reqwest::Client::new();

    let resp = client
        .post(format!("http://{addr}/api/v1/validate-account"))
        .json(&json!({
            "game_code": "mlbb",
            "customer_target": "12345678",
            "customer_target_zone": "1234"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["result"], true);
    assert_eq!(body["data"]["username"], "Gamer123");
}

#[tokio::test]
async fn test_create_and_get_order() {
    let mock = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/h2h/transaction"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "success": true,
            "data": {
                "id": "11111111-1111-4111-8111-111111111111",
                "buyer_trx_id": "ODGJ-ORDER-1",
                "sku_code": "MLBB-50",
                "customer_target": "12345678",
                "customer_target_zone": "1234",
                "price": 14000.0,
                "status": "success",
                "serial_number": "SN12345678",
                "webhook_delivery_status": "pending",
                "is_sandbox": true,
                "created_at": "2026-10-02T03:00:00+00:00"
            }
        })))
        .mount(&mock)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/h2h/transaction/11111111-1111-4111-8111-111111111111"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "data": {
                "id": "11111111-1111-4111-8111-111111111111",
                "buyer_trx_id": "ODGJ-ORDER-1",
                "sku_code": "MLBB-50",
                "customer_target": "12345678",
                "customer_target_zone": "1234",
                "price": 14000.0,
                "status": "success",
                "serial_number": "SN12345678",
                "webhook_delivery_status": "delivered",
                "is_sandbox": true,
                "created_at": "2026-10-02T03:00:00+00:00"
            }
        })))
        .mount(&mock)
        .await;

    let (addr, _handle) = spawn_test_server(&mock.uri()).await;
    let client = reqwest::Client::new();

    // 1. Create order
    let resp = client
        .post(format!("http://{addr}/api/v1/orders"))
        .json(&json!({
            "sku_code": "MLBB-50",
            "customer_target": "12345678",
            "buyer_trx_id": "ODGJ-ORDER-1"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["id"], "11111111-1111-4111-8111-111111111111");
    assert_eq!(body["status"], "success");

    // 2. Get order status
    let resp2 = client
        .get(format!(
            "http://{addr}/api/v1/orders/11111111-1111-4111-8111-111111111111"
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(resp2.status(), StatusCode::OK);
    let body2: serde_json::Value = resp2.json().await.unwrap();
    assert_eq!(body2["id"], "11111111-1111-4111-8111-111111111111");
    assert_eq!(body2["status"], "success");
}
