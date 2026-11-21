pub mod health;
pub mod orders;
pub mod products;
pub mod validate;

use crate::AppState;
use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn router(state: AppState) -> Router {
    let api = Router::new()
        .route("/health", get(health::health_handler))
        .route("/products", get(products::get_products))
        .route("/products/{category_slug}", get(products::get_products_by_category))
        .route("/orders", post(orders::create_order))
        .route("/orders/{id}", get(orders::get_order_status))
        .route("/validate-account", post(validate::validate_account));

    let cors = if state.cfg.cors_origin == "*" {
        CorsLayer::permissive()
    } else {
        let allowed = HeaderValue::from_str(&state.cfg.cors_origin)
            .unwrap_or_else(|_| HeaderValue::from_static("*"));
        CorsLayer::new()
            .allow_origin(allowed)
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
    };

    Router::new()
        .route("/health", get(health::health_handler))
        .nest("/api/v1", api)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
