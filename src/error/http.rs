use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub fn bad_request(msg: &str) -> Response {
    (StatusCode::BAD_REQUEST, Json(json!({"error" : msg }))).into_response()
}

pub fn conflict(msg: &str) -> Response {
    (StatusCode::CONFLICT, Json(json!({"error" : msg}))).into_response()
}

pub fn unavailable(msg: &str) -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({"error" : msg})),
    )
        .into_response()
}

pub fn internal_error(msg: &str) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error" : msg})),
    )
        .into_response()
}

pub fn unprocessable_entity(msg: &str) -> Response {
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(json!({"error" : msg})),
    )
        .into_response()
}

pub fn not_found(msg: &str) -> Response {
    (StatusCode::NOT_FOUND, Json(json!({"error" : msg}))).into_response()
}
