use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    app::state::AppState,
    features::room::handlers::{create_room, delete_room, list_room},
};

pub fn room_routes() -> Router<AppState> {
    Router::new()
        .route("/room", post(create_room))
        .route("/room", get(list_room))
        .route("/room", delete(delete_room))
}
