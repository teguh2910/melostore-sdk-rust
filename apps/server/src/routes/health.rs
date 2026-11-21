use crate::AppState;
use axum::{extract::State, Json};
use serde_json::{json, Value};

pub async fn health_handler(State(state): State<AppState>) -> Json<Value> {
    match state.client.get_balance().await {
        Ok(bal) => Json(json!({
            "status": "ok",
            "provider": {
                "connected": true,
                "balance": bal.h2h_balance,
                "is_sandbox": bal.is_sandbox_mode,
            }
        })),
        Err(err) => Json(json!({
            "status": "degraded",
            "provider": {
                "connected": false,
                "error": err.to_string(),
            }
        })),
    }
}
