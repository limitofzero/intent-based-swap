use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("validation: {0}")]
    Validation(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use axum::{Json, http::StatusCode};
        let (status, code) = match &self {
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::Internal(mgs) => (StatusCode::INTERNAL_SERVER_ERROR, mgs),
        };
        let body = serde_json::json!({ "error": { "code": code, "message": self.to_string() }});
        (status, Json(body)).into_response()
    }
}
