use crate::{error::AppError, AppState};
use axum::{extract::State, Json};
use melostore_sdk_rust::models::account::CheckNicknameRequest;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct ValidateAccountPayload {
    pub game_code: Option<String>,
    pub customer_target: Option<String>,
    pub uid: Option<String>,
    pub customer_target_zone: Option<String>,
    pub zone: Option<String>,
}

pub async fn validate_account(
    State(state): State<AppState>,
    Json(payload): Json<ValidateAccountPayload>,
) -> Result<Json<Value>, AppError> {
    let game_code = payload
        .game_code
        .ok_or_else(|| AppError::BadRequest("Field 'game_code' is required".into()))?;

    let customer_target = payload
        .customer_target
        .or(payload.uid)
        .ok_or_else(|| AppError::BadRequest("Field 'customer_target' is required".into()))?;

    let customer_target_zone = payload.customer_target_zone.or(payload.zone);

    let mut req = CheckNicknameRequest::new(game_code, customer_target);
    req.customer_target_zone = customer_target_zone;

    let res = state.client.check_nickname(req).await?;
    Ok(Json(json!({
        "result": res.success,
        "data": res.data,
        "billing": res.billing,
        "message": "success",
    })))
}
