use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    app::state::AppState,
    features::book::handlers::{create_booking, delete_book, list_book},
};

pub fn book_routes() -> Router<AppState> {
    Router::new()
        .route("/book", post(create_booking))
        .route("/book", get(list_book))
        .route("/book", delete(delete_book))
}
