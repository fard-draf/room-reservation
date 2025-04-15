use std::{error::Error, sync::Arc};
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;

use crate::{
    app::state::AppState, features::{
        booking::service::BookingService,
        room::service::RoomService,
        user::service::UserService,
    }, infra::db::DBClient
};

pub async fn build_app(database_url: &str) -> Result<Router, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    let db_client = DBClient::new(pool);

    let state = AppState {
        user_service: Arc::new(Mutex::new(UserService::new(db_client.clone()))),
        room_service: Arc::new(Mutex::new(BookingService::new(db_client.clone()))),
        book_service: Arc::new(Mutex::new(RoomService::new(db_client.clone())))
    };

    let app = Router::new()
        .merge(user_routes())
        .merge(room_routes())
        .merge(reg_routes())
        .with_state(state);

    Ok(app)
}
