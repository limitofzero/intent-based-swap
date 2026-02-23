use crate::AppState;
use crate::api::errors::AppError;
use crate::domain::quote::{QuoteRequest, QuoteResponse};
use crate::services::quoter::QuoterError;
use axum::Json;
use axum::extract::State;
use std::sync::Arc;

pub async fn get_quote(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<QuoteRequest>,
) -> Result<Json<QuoteResponse>, AppError> {
    match state.quoter.get_quote(payload).await {
        Ok(quote) => Ok(Json(quote)),
        Err(e) => match e {
            QuoterError::InvalidQuoteRequest(msg) => Err(AppError::Validation(msg)),
            QuoterError::FailedToGetPrice(msg) => Err(AppError::Internal(msg)),
        },
    }
}
