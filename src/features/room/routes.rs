use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    app::state::AppState,
    features::room::handlers::{create_room, delete_room, list_room},
};

use super::handlers::update_room_name;

pub fn room_routes() -> Router<AppState> {
    Router::new()
        .route("/room", post(create_room))
        .route("/room/update", post(update_room_name))
        .route("/room", get(list_room))
        .route("/room", delete(delete_room))
}
