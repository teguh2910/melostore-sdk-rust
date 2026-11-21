use serde_json::{json, Value};

pub fn get_mock_health() -> Value {
    json!({
        "status": "ok",
        "provider": {
            "connected": true,
            "balance": 10000000.0,
            "is_sandbox": true
        }
    })
}

pub fn get_mock_catalogue(category_slug: Option<&str>) -> Value {
    let all_products = vec![
        // Mobile Legends
        json!({
            "name": "Mobile Legends 86 Diamonds",
            "sku_code": "ML-86",
            "price": 19500.0,
            "category_name": "Mobile Legends",
            "brand_id": 1,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "mobile-legends"
        }),
        json!({
            "name": "Mobile Legends 172 Diamonds",
            "sku_code": "ML-172",
            "price": 38000.0,
            "category_name": "Mobile Legends",
            "brand_id": 1,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "mobile-legends"
        }),
        json!({
            "name": "Mobile Legends 257 Diamonds",
            "sku_code": "ML-257",
            "price": 57000.0,
            "category_name": "Mobile Legends",
            "brand_id": 1,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "mobile-legends"
        }),
        json!({
            "name": "Weekly Diamond Pass",
            "sku_code": "ML-WDP",
            "price": 27500.0,
            "category_name": "Mobile Legends",
            "brand_id": 1,
            "type_name": "Subscription",
            "status": "active",
            "slug": "mobile-legends"
        }),

        // Free Fire
        json!({
            "name": "Free Fire 70 Diamonds",
            "sku_code": "FF-70",
            "price": 9500.0,
            "category_name": "Free Fire",
            "brand_id": 291,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "free-fire-indonesia-3"
        }),
        json!({
            "name": "Free Fire 140 Diamonds",
            "sku_code": "FF-140",
            "price": 19000.0,
            "category_name": "Free Fire",
            "brand_id": 291,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "free-fire-indonesia-3"
        }),
        json!({
            "name": "Free Fire 355 Diamonds",
            "sku_code": "FF-355",
            "price": 47000.0,
            "category_name": "Free Fire",
            "brand_id": 291,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "free-fire-indonesia-3"
        }),

        // Genshin Impact
        json!({
            "name": "Blessing of the Welkin Moon",
            "sku_code": "GI-WELKIN",
            "price": 75000.0,
            "category_name": "Genshin Impact",
            "brand_id": 479,
            "type_name": "Subscription",
            "status": "active",
            "slug": "genshin-impact-id-1"
        }),
        json!({
            "name": "Genshin Impact 60 Genesis Crystals",
            "sku_code": "GI-60",
            "price": 14500.0,
            "category_name": "Genshin Impact",
            "brand_id": 479,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "genshin-impact-id-1"
        }),
        json!({
            "name": "Genshin Impact 300+30 Genesis Crystals",
            "sku_code": "GI-300",
            "price": 70000.0,
            "category_name": "Genshin Impact",
            "brand_id": 613,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "genshin-impact-global-7"
        }),

        // Honkai Star Rail
        json!({
            "name": "Express Supply Pass",
            "sku_code": "HSR-EXPRESS",
            "price": 75000.0,
            "category_name": "Honkai: Star Rail",
            "brand_id": 480,
            "type_name": "Subscription",
            "status": "active",
            "slug": "honkai-star-rail-1-2"
        }),
        json!({
            "name": "Honkai: Star Rail 60 Oneiric Shards",
            "sku_code": "HSR-60",
            "price": 14500.0,
            "category_name": "Honkai: Star Rail",
            "brand_id": 480,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "honkai-star-rail-1-2"
        }),
        json!({
            "name": "Honkai: Star Rail 300+30 Oneiric Shards",
            "sku_code": "HSR-300",
            "price": 70000.0,
            "category_name": "Honkai: Star Rail",
            "brand_id": 614,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "honkai-star-rail-5"
        }),

        // Zenless Zone Zero
        json!({
            "name": "ZZZ 60 Monochrome",
            "sku_code": "ZZZ-60",
            "price": 14500.0,
            "category_name": "Zenless Zone Zero",
            "brand_id": 481,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "zenless-zone-zero-id-1"
        }),
        json!({
            "name": "ZZZ 300+30 Monochrome",
            "sku_code": "ZZZ-300",
            "price": 70000.0,
            "category_name": "Zenless Zone Zero",
            "brand_id": 615,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "zenless-zone-zero-global-7"
        }),

        // Magic Chess: Go Go
        json!({
            "name": "Magic Chess: Go Go 50 Diamonds",
            "sku_code": "MCGG-50",
            "price": 10000.0,
            "category_name": "Magic Chess: Go Go",
            "brand_id": 4754,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "magic-chess-gogo-10"
        }),
        json!({
            "name": "Magic Chess: Go Go 100 Diamonds",
            "sku_code": "MCGG-100",
            "price": 20000.0,
            "category_name": "Magic Chess: Go Go",
            "brand_id": 651,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "magic-chess-go-go-indonesia-6"
        }),

        // Valorant
        json!({
            "name": "Valorant 125 Points",
            "sku_code": "VAL-125",
            "price": 15000.0,
            "category_name": "Valorant",
            "brand_id": 599,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "valorant-id-7"
        }),
        json!({
            "name": "Valorant 420 Points",
            "sku_code": "VAL-420",
            "price": 50000.0,
            "category_name": "Valorant",
            "brand_id": 2567,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "valorant-indonesia-1"
        }),

        // PUBG Mobile
        json!({
            "name": "PUBG Mobile 60 UC",
            "sku_code": "PUBG-60",
            "price": 14000.0,
            "category_name": "PUBG Mobile",
            "brand_id": 3081,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "pubg-mobile-id-5"
        }),
        json!({
            "name": "PUBG Mobile 325 UC",
            "sku_code": "PUBG-325",
            "price": 70000.0,
            "category_name": "PUBG Mobile",
            "brand_id": 302,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "pubg-mobile-global-3"
        }),

        // Honor of Kings
        json!({
            "name": "Honor of Kings 80 Tokens",
            "sku_code": "HOK-80",
            "price": 15000.0,
            "category_name": "Honor of Kings",
            "brand_id": 680,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "honor-of-kings-7"
        }),

        // Call of Duty Mobile
        json!({
            "name": "CODM 31 CP",
            "sku_code": "CODM-31",
            "price": 5000.0,
            "category_name": "Call of Duty Mobile",
            "brand_id": 3767,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "call-of-duty-mobile-indonesia-id-2-3"
        }),
        json!({
            "name": "CODM 62 CP",
            "sku_code": "CODM-62",
            "price": 10000.0,
            "category_name": "Call of Duty Mobile",
            "brand_id": 3804,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "call-of-duty-mobile-13"
        }),

        // Zepeto & AFK Journey
        json!({
            "name": "Zepeto 14 ZEM",
            "sku_code": "ZEP-14",
            "price": 10000.0,
            "category_name": "Zepeto",
            "brand_id": 76,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "zepeto"
        }),
        json!({
            "name": "AFK Journey 60 Dragon Crystals",
            "sku_code": "AFK-60",
            "price": 15000.0,
            "category_name": "AFK Journey",
            "brand_id": 150,
            "type_name": "Direct Topup",
            "status": "active",
            "slug": "afk-journey"
        }),

        // Pulsa & PLN
        json!({
            "name": "Telkomsel 10.000",
            "sku_code": "TSEL-10K",
            "price": 10500.0,
            "category_name": "Pulsa",
            "brand_id": 12,
            "type_name": "Pulsa Reguler",
            "status": "active",
            "slug": "telkomsel-12"
        }),
        json!({
            "name": "Indosat 10.000",
            "sku_code": "ISAT-10K",
            "price": 10500.0,
            "category_name": "Pulsa",
            "brand_id": 13,
            "type_name": "Pulsa Reguler",
            "status": "active",
            "slug": "indosat-12"
        }),
        json!({
            "name": "XL 10.000",
            "sku_code": "XL-10K",
            "price": 10500.0,
            "category_name": "Pulsa",
            "brand_id": 9,
            "type_name": "Pulsa Reguler",
            "status": "active",
            "slug": "xl-reguler-ss-9"
        }),
        json!({
            "name": "Axis 10.000",
            "sku_code": "AXIS-10K",
            "price": 10500.0,
            "category_name": "Pulsa",
            "brand_id": 14,
            "type_name": "Pulsa Reguler",
            "status": "active",
            "slug": "axis-13"
        }),
        json!({
            "name": "By.U 10.000",
            "sku_code": "BYU-10K",
            "price": 10500.0,
            "category_name": "Pulsa",
            "brand_id": 15,
            "type_name": "Pulsa Reguler",
            "status": "active",
            "slug": "byu-12"
        }),
        json!({
            "name": "Tri 10.000",
            "sku_code": "TRI-10K",
            "price": 10500.0,
            "category_name": "Pulsa",
            "brand_id": 4,
            "type_name": "Pulsa Reguler",
            "status": "active",
            "slug": "3-4"
        }),
        json!({
            "name": "Smartfren 10.000",
            "sku_code": "SMART-10K",
            "price": 10500.0,
            "category_name": "Pulsa",
            "brand_id": 10,
            "type_name": "Pulsa Reguler",
            "status": "active",
            "slug": "smartfren-10"
        }),
        json!({
            "name": "PLN Token 20.000",
            "sku_code": "PLN-20K",
            "price": 20500.0,
            "category_name": "PLN",
            "brand_id": 20,
            "type_name": "Token Listrik",
            "status": "active",
            "slug": "pln-prepaid"
        })
    ];

    let filtered: Vec<Value> = if let Some(slug) = category_slug {
        let matching: Vec<Value> = all_products
            .into_iter()
            .filter(|p| {
                if let Some(s) = p.get("slug").and_then(|v| v.as_str()) {
                    s == slug || slug.contains(s) || s.contains(slug)
                } else {
                    false
                }
            })
            .collect();
        if matching.is_empty() {
            vec![json!({
                "name": format!("Item for {slug}"),
                "sku_code": format!("MELO-{}", slug.to_uppercase()),
                "price": 15000.0,
                "category_name": slug,
                "status": "active"
            })]
        } else {
            matching
        }
    } else {
        all_products
    };

    json!({
        "success": true,
        "data": filtered,
        "meta": {
            "brands": [
                { "id": 1, "name": "Mobile Legends", "slug": "mobile-legends" },
                { "id": 291, "name": "Free Fire", "slug": "free-fire-indonesia-3" },
                { "id": 479, "name": "Genshin Impact", "slug": "genshin-impact-id-1" },
                { "id": 613, "name": "Genshin Impact Global", "slug": "genshin-impact-global-7" },
                { "id": 480, "name": "Honkai: Star Rail", "slug": "honkai-star-rail-1-2" },
                { "id": 614, "name": "Honkai: Star Rail Global", "slug": "honkai-star-rail-5" },
                { "id": 481, "name": "Zenless Zone Zero", "slug": "zenless-zone-zero-id-1" },
                { "id": 615, "name": "Zenless Zone Zero Global", "slug": "zenless-zone-zero-global-7" },
                { "id": 12, "name": "Telkomsel", "slug": "telkomsel-12" },
                { "id": 13, "name": "Indosat", "slug": "indosat-12" },
                { "id": 9, "name": "XL", "slug": "xl-reguler-ss-9" },
                { "id": 14, "name": "Axis", "slug": "axis-13" },
                { "id": 15, "name": "By.U", "slug": "byu-12" },
                { "id": 4, "name": "Tri", "slug": "3-4" },
                { "id": 10, "name": "Smartfren", "slug": "smartfren-10" },
                { "id": 20, "name": "PLN", "slug": "pln-prepaid" },
                { "id": 4754, "name": "Magic Chess: Go Go", "slug": "magic-chess-gogo-10" },
                { "id": 651, "name": "Magic Chess: Go Go ID", "slug": "magic-chess-go-go-indonesia-6" },
                { "id": 599, "name": "Valorant ID", "slug": "valorant-id-7" },
                { "id": 2567, "name": "Valorant Indonesia", "slug": "valorant-indonesia-1" },
                { "id": 3081, "name": "PUBG Mobile ID", "slug": "pubg-mobile-id-5" },
                { "id": 302, "name": "PUBG Mobile Global", "slug": "pubg-mobile-global-3" },
                { "id": 680, "name": "Honor of Kings", "slug": "honor-of-kings-7" },
                { "id": 3767, "name": "Call of Duty Mobile ID", "slug": "call-of-duty-mobile-indonesia-id-2-3" },
                { "id": 3804, "name": "Call of Duty Mobile", "slug": "call-of-duty-mobile-13" },
                { "id": 76, "name": "Zepeto", "slug": "zepeto" },
                { "id": 150, "name": "AFK Journey", "slug": "afk-journey" }
            ],
            "inquiry_forms": {},
            "pagination": {
                "has_more": false,
                "next_cursor": null
            }
        }
    })
}

pub fn create_mock_order(buyer_trx_id: &str, sku_code: &str, _customer_target: &str) -> Value {
    json!({
        "id": format!("MELO-MOCK-{}", &buyer_trx_id[..buyer_trx_id.len().min(12)]),
        "buyer_trx_id": buyer_trx_id,
        "sku_code": sku_code,
        "price": 15000.0,
        "status": "success",
        "serial_number": format!("SN-MELO-{}", &buyer_trx_id[..buyer_trx_id.len().min(8)]),
        "message": "Order processed successfully (mock mode)"
    })
}

pub fn get_mock_order_status(id: &str) -> Value {
    json!({
        "id": id,
        "buyer_trx_id": id,
        "sku_code": "MOCK-SKU",
        "price": 15000.0,
        "status": "success",
        "serial_number": format!("SN-MELO-{}", &id[..id.len().min(8)]),
        "message": "Order status fetched (mock mode)"
    })
}

pub fn validate_mock_account(customer_target: &str, customer_target_zone: Option<&str>) -> Value {
    json!({
        "result": true,
        "data": {
            "username": format!("Player_{customer_target}"),
            "customer_target_zone": customer_target_zone.unwrap_or("Server 1")
        },
        "message": "Account validated (mock mode)"
    })
}
