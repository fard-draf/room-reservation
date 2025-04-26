use axum::{Router, middleware::from_fn};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    app::{state::AppState, status_test::log_status},
    error::ErrService,
    features::{
        book::{routes::book_routes, service::BookService},
        room::{routes::room_routes, service::RoomService},
        user::{routes::user_routes, service::UserService},
    },
    infra::{cache::try_init_caches, db::DBClient},
};

pub async fn build_app(database_url: &str) -> Result<Router, ErrService> {
    let pool = PgPoolOptions::new()
        .max_connections(30)
        .connect(database_url)
        .await?;

    let db_client = DBClient::new(pool);

    let room_service = Arc::new(RoomService::new(db_client.clone()));
    let user_service = Arc::new(UserService::new(db_client.clone()));
    let book_service = Arc::new(BookService::new(db_client.clone()));

    try_init_caches(&user_service, &room_service, &book_service).await?;

    let state = AppState {
        user_service: user_service.clone(),
        room_service: room_service.clone(),
        book_service: book_service.clone(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(book_routes())
        .merge(room_routes())
        .merge(user_routes())
        .with_state(state)
        .layer(cors)
        .layer(from_fn(log_status));

    Ok(app)
}
