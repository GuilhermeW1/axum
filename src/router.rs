use axum::Router;

use crate::{AppState, handler::user_handler::user_router};

pub fn routes() -> Router<AppState> {
    Router::new().merge(user_router())
}
