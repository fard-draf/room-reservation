use axum::{
   routing::{delete, get, post}, Router
};

use crate::{
    features::user::
        handlers::{
            create_user,
            delete_user,
            list_users
            },
        app::state::AppState,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(list_users))
        .route("/users", delete(delete_user))
}
