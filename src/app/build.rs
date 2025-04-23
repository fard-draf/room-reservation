use axum::{Router, middleware::from_fn};
use sqlx::postgres::PgPoolOptions;
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::{
    app::{state::AppState, status_test::log_status},
    features::{
        book::{routes::book_routes, service::BookService},
        room::{routes::room_routes, service::RoomService},
        user::{routes::user_routes, service::UserService},
    },
    infra::db::DBClient,
};

pub async fn build_app(database_url: &str) -> Result<Router, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;

    let db_client = DBClient::new(pool);

    let state = AppState {
        user_service: Arc::new(Mutex::new(UserService::new(db_client.clone()))),
        room_service: Arc::new(Mutex::new(RoomService::new(db_client.clone()))),
        book_service: Arc::new(Mutex::new(BookService::new(db_client.clone()))),
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
