use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    app::state::AppState,
    features::user::handlers::{create_user, delete_user, list_users},
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(list_users))
        .route("/users", delete(delete_user))
}
