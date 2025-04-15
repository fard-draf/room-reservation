use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    infra::db::DBClient, 
    features::{
        booking::service::BookingService,
        room::service::RoomService,
        user::service::UserService
    }
};

pub type SharedUserService = Arc<Mutex<UserService<DBClient>>>;
pub type SharedRoomService = Arc<Mutex<RoomService<DBClient>>>;
pub type SharedRegService = Arc<Mutex<BookingService<DBClient>>>;

#[derive(Clone)]
pub struct AppState {
    pub user_service: SharedUserService,
    pub room_service: SharedRoomService,
    pub book_service: SharedRegService,
}
