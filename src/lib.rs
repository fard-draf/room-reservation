pub mod domain;
pub mod error;
pub mod infra;
pub mod repository;
pub mod tests;

pub use domain::{Book, Room, User};
pub use error::ErrService;
pub use infra::{
    in_memo_repo::InMemoryRepo, reg_service::RegService, room_service::RoomService,
    user_service::UserService,
};