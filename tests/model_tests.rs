use std::collections::HashMap;
use melostore_sdk_rust::models::account::{NicknameResult, PurchaseLimitResult};
use melostore_sdk_rust::models::catalog::{Catalogue, PricelistFormat, PricelistParams, SmartCatalogue};
use melostore_sdk_rust::models::common::ApiResponse;
use melostore_sdk_rust::models::profile::Profile;
use melostore_sdk_rust::models::transaction::{CreateSmartTransactionRequest, CreateTransactionRequest};
use melostore_sdk_rust::{Environment, MelostoreConfig, MelostoreError};

#[test]
fn test_config_builder_validation() {
    // Missing API key
    let err = MelostoreConfig::builder()
        .secret_key("secret")
        .build()
        .unwrap_err();
    assert!(matches!(err, MelostoreError::MissingRequiredField(_)));

    // Missing Secret key
    let err = MelostoreConfig::builder()
        .api_key("key")
        .build()
        .unwrap_err();
    assert!(matches!(err, MelostoreError::MissingRequiredField(_)));

    // Empty key
    let err = MelostoreConfig::builder()
        .api_key("   ")
        .secret_key("secret")
        .build()
        .unwrap_err();
    assert!(matches!(err, MelostoreError::InvalidParameter(_)));

    // Trailing slash in base_url
    let config = MelostoreConfig::builder()
        .api_key("key")
        .secret_key("secret")
        .base_url("https://custom.api.id/")
        .build()
        .unwrap();
    assert_eq!(config.base_url, "https://custom.api.id");

    // Default environment
    let config = MelostoreConfig::new("key", "secret");
    assert_eq!(config.base_url, Environment::Production.base_url());
}

#[test]
fn test_profile_deserialization() {
    let json_data = r#"{
        "success": true,
        "data": {
            "id": "11111111-1111-4111-8111-111111111111",
            "name": "Example Partner",
            "email": "partner@example.com",
            "tier": {
                "name": "Basic",
                "code": "basic",
                "rate_limit": 30,
                "max_ips": 3
            },
            "h2h_balance": 100000,
            "is_sandbox_mode": true,
            "total_volume_30_days": 5000,
            "webhook_url": "https://partner.example.com/h2h-callback",
            "created_at": "2026-10-02T03:00:00+00:00",
            "usd_idr_rate": 17000,
            "display_usd_idr_rate": 17000,
            "display_rate_updated_at": "2026-10-02T03:00:00+00:00"
        }
    }"#;

    let res: ApiResponse<Profile> = serde_json::from_str(json_data).expect("Failed to parse Profile");
    assert!(res.success);
    assert_eq!(res.data.name, "Example Partner");
    assert_eq!(res.data.tier.code, "basic");
    assert_eq!(res.data.h2h_balance, 100000.0);
    assert!(res.data.is_sandbox_mode);
}

#[test]
fn test_catalogue_inquiry_forms_empty_array() {
    // In Laravel/PHP, empty assoc arrays serialize as [] in JSON instead of {}
    let json_data = r#"{
        "success": true,
        "data": [
            {
                "name": "Weekly Diamond Pass",
                "sku_code": "MLBB-WDP",
                "price": 27500,
                "category_name": "Mobile Legends",
                "brand_id": 1,
                "type_name": "Membership",
                "status": "active"
            }
        ],
        "meta": {
            "brands": [],
            "inquiry_forms": []
        }
    }"#;

    let cat: Catalogue = serde_json::from_str(json_data).expect("Must parse empty array inquiry_forms");
    assert_eq!(cat.data.len(), 1);
    assert!(cat.meta.inquiry_forms.is_empty());
}

#[test]
fn test_catalogue_inquiry_forms_with_fields() {
    let json_data = r#"{
        "success": true,
        "data": [],
        "meta": {
            "brands": [],
            "inquiry_forms": {
                "mobile_legends": {
                    "fields": [
                        {
                            "key": "user_id",
                            "name": "User ID",
                            "type": "text",
                            "required": true,
                            "placeholder": "Masukkan User ID"
                        },
                        {
                            "key": "zone_id",
                            "name": "Zone ID",
                            "type": "text",
                            "required": true,
                            "placeholder": "Masukkan Zone ID"
                        }
                    ],
                    "notice": {
                        "id": "Masukkan User ID dan Zone ID MLBB Anda",
                        "en": "Enter your MLBB User ID and Zone ID"
                    }
                }
            }
        }
    }"#;

    let cat: Catalogue = serde_json::from_str(json_data).expect("Must parse populated inquiry_forms");
    assert_eq!(cat.meta.inquiry_forms.len(), 1);
    let form = &cat.meta.inquiry_forms["mobile_legends"];
    assert_eq!(form.fields.len(), 2);
    assert_eq!(form.fields[0].key, "user_id");
    assert!(form.fields[0].required);
    assert_eq!(form.notice.as_ref().unwrap().id.as_deref(), Some("Masukkan User ID dan Zone ID MLBB Anda"));
}

#[test]
fn test_smart_catalogue_deserialization() {
    let json_data = r#"{
        "success": true,
        "data": [
            {
                "name": "Weekly Diamond Pass",
                "category_name": "Mobile Legends",
                "status": "active",
                "smart_sku_code": "SMART-WDP",
                "min_price": 27000,
                "max_price": 28000,
                "variant_count": 2,
                "variants": [
                    { "sku_code": "WDP-SUPPLIER-1", "price": 27000, "status": "active" },
                    { "sku_code": "WDP-SUPPLIER-2", "price": 28000, "status": "active" }
                ]
            }
        ],
        "meta": {
            "brands": [],
            "inquiry_forms": {}
        }
    }"#;

    let smart: SmartCatalogue = serde_json::from_str(json_data).expect("Must parse SmartCatalogue");
    assert_eq!(smart.data.len(), 1);
    assert_eq!(smart.data[0].smart_sku_code, "SMART-WDP");
    assert_eq!(smart.data[0].min_price, Some(27000.0));
    assert_eq!(smart.data[0].max_price, Some(28000.0));
    assert_eq!(smart.data[0].variants.len(), 2);
}

#[test]
fn test_transaction_request_serialization() {
    let mut req = CreateTransactionRequest::new("MLBB-86", "12345678", "buyer-ref-001");
    req.customer_target_zone = Some("1234".into());
    req.max_price = Some(20000);
    req.sandbox_mode = Some(true);

    let mut add = HashMap::new();
    add.insert("server".to_string(), serde_json::json!("asia"));
    req.additional_data = Some(add);

    let serialized = serde_json::to_string(&req).expect("Failed to serialize");
    assert!(serialized.contains("\"sku_code\":\"MLBB-86\""));
    assert!(serialized.contains("\"customer_target_zone\":\"1234\""));
    assert!(serialized.contains("\"max_price\":20000"));
    assert!(serialized.contains("\"sandbox_mode\":true"));
}

#[test]
fn test_smart_transaction_request_serialization() {
    let req = CreateSmartTransactionRequest::new("SMART-WDP", "987654", "smart-ref-100", 28500);
    let serialized = serde_json::to_string(&req).expect("Failed to serialize");
    assert!(serialized.contains("\"max_bid\":28500"));
    assert!(serialized.contains("\"sku_code\":\"SMART-WDP\""));
}

#[test]
fn test_pricelist_params_serialization() {
    let params = PricelistParams {
        limit: Some(500),
        format: Some(PricelistFormat::Compact),
        server: Some("asia".into()),
        brand_id: Some(10),
        ..Default::default()
    };

    let json_str = serde_json::to_string(&params).expect("Failed to serialize query params");
    assert!(json_str.contains("\"limit\":500"));
    assert!(json_str.contains("\"format\":\"compact\""));
    assert!(json_str.contains("\"server\":\"asia\""));
    assert!(json_str.contains("\"brand_id\":10"));
}

#[test]
fn test_nickname_result_deserialization() {
    let json_data = r#"{
        "success": true,
        "data": {
            "game_code": "mobile_legends",
            "customer_target": "12345678",
            "customer_target_zone": "1234",
            "username": "HeroLegend",
            "region": "ID",
            "region_type": "national"
        },
        "billing": {
            "charged_mc": 0,
            "free_remaining_today": 100,
            "transaction_id": null
        }
    }"#;

    let res: NicknameResult = serde_json::from_str(json_data).expect("Failed to parse NicknameResult");
    assert!(res.success);
    assert_eq!(res.data.username, "HeroLegend");
    assert_eq!(res.billing.free_remaining_today, Some(100));
}

#[test]
fn test_purchase_limit_deserialization() {
    let json_data = r#"{
        "success": true,
        "data": {
            "game_code": "mobile_legends",
            "customer_target": "12345678",
            "customer_target_zone": "1234",
            "username": "HeroLegend",
            "weekly_pass": {
                "limit_reached": false,
                "items": [
                    { "package_code": "WDP-01", "limit_reached": false }
                ]
            },
            "double_diamonds": {
                "eligible": true,
                "items": [
                    { "package_code": "DD-50", "limit_reached": false }
                ]
            }
        }
    }"#;

    let res: PurchaseLimitResult = serde_json::from_str(json_data).expect("Failed to parse PurchaseLimitResult");
    assert!(res.success);
    assert!(!res.data.weekly_pass.unwrap().limit_reached);
    assert!(res.data.double_diamonds.unwrap().eligible);
}
