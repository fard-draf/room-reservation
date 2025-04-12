use std::error::Error;

use domain::{Book, Room, User};
use infra::{
    in_memo_repo::InMemoryRepo, reg_service::RegService, room_service::RoomService,
    user_service::UserService,
};


mod domain;
mod infra;
mod repository;
mod run;
mod tests;
mod error;

fn main() -> Result<(), Box<dyn Error>> {
    let repo_room: InMemoryRepo<Room> = InMemoryRepo::new();
    let mut room_service: RoomService<InMemoryRepo<Room>> = RoomService::new(repo_room);

    let repo_reg: InMemoryRepo<Book> = InMemoryRepo::new();
    let mut reg_service: RegService<InMemoryRepo<Book>> = RegService::new(repo_reg);

    let repo_user: InMemoryRepo<User> = InMemoryRepo::new();
    let mut user_service: UserService<InMemoryRepo<User>> = UserService::new(repo_user);

    Ok(())
}
