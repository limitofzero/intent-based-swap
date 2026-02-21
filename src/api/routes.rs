use crate::AppState;
use crate::handlers::quote::get_quote;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn get_router(app_state: AppState) -> Router {
    let shared_state = Arc::new(app_state);

    Router::new()
        .route("/quote", post(get_quote))
        .with_state(shared_state)
}
