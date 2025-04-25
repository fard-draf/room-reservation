use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    features::{
        book::service::BookService, room::service::RoomService, user::service::UserService,
    },
    infra::db::DBClient,
};

pub type SharedUserService = Arc<Mutex<UserService<DBClient>>>;
pub type SharedRoomService = Arc<RoomService<DBClient>>;
pub type SharedBookService = Arc<BookService<DBClient>>;

#[derive(Clone)]
pub struct AppState {
    pub user_service: SharedUserService,
    pub room_service: SharedRoomService,
    pub book_service: SharedBookService,
}
