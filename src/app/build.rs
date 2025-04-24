use axum::{Router, middleware::from_fn};
use sqlx::postgres::PgPoolOptions;
use std::{collections::HashSet, error::Error, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tower_http::cors::{Any, CorsLayer};

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

    let room_service = Arc::new(RoomService::new(db_client.clone()));
    let user_service = Arc::new(UserService::new(db_client.clone()));
    let book_service = Arc::new(Mutex::new(BookService {
        repo: db_client.clone(),
        user_service: Arc::clone(&user_service),
        room_service: Arc::clone(&room_service),
        cache: RwLock::new(HashSet::new()),
    }));
    {
        let room_cache = Arc::clone(&room_service).populate_cache().await;
        println!("{:?}", room_cache);

        let user_cache = Arc::clone(&user_service);
        user_cache.populate_cache().await;
        println!("{:?}", user_cache);

        let mut book_cache = book_service.lock().await;
        book_cache.populate_cache().await;
        println!("{:?}", book_cache);
    }

    let state = AppState {
        user_service,
        room_service,
        book_service,
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
