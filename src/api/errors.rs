use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("validation: {0}")]
    Validation(String),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use axum::{Json, http::StatusCode};
        let (status, code) = match &self {
            AppError::Validation(_) => (StatusCode::UNPROCESSABLE_ENTITY, "VALIDATION"),
        };
        let body = serde_json::json!({ "error": { "code": code, "message": self.to_string() }});
        (status, Json(body)).into_response()
    }
}
