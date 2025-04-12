use std::error::Error;

use domain::{Book, Room, User};
use infra::{
    in_memo_repo::InMemoryRepo, reg_service::RegService, room_service::RoomService,
    user_service::UserService,
};
pub mod domain;
pub mod infra;
mod repository;
mod error;
mod test;
mod run;

fn main() -> Result<(), Box<dyn Error>> {
    let repo_room: InMemoryRepo<Room> = InMemoryRepo::new();
    let mut room_service: RoomService<InMemoryRepo<Room>> = RoomService::new(repo_room);

    let repo_reg: InMemoryRepo<Book> = InMemoryRepo::new();
    let mut reg_service: RegService<InMemoryRepo<Book>> = RegService::new(repo_reg);

    let repo_user: InMemoryRepo<User> = InMemoryRepo::new();
    let mut user_service: UserService<InMemoryRepo<User>> = UserService::new(repo_user);



    let francois = user_service.add_user("Francois Fouesn")?;
    let jeanne = user_service.add_user("Jeanne Delcros")?;
    let david = user_service.add_user("David Durand")?;
    println!(" User's list: {:#?}",user_service.list_users());

    let room1 = room_service.add_room("Suite Royale")?;
    let room2 = room_service.add_room("Etape nocturne")?;
    let room3 = room_service.add_room("Dolce Note")?;
    let room4 = room_service.add_room("Black room")?;

    println!("Is this room exist? {:#?}",room_service.is_exist_room("El camino"));
    println!("Is this room exist? {:#?}",room_service.is_exist_room("Dolce Note"));
    println!("Room's list: {:#?}",room_service.list_rooms());

    reg_service.book_room(&room1, &francois, "20.10.02")?;
    reg_service.book_room(&room2, &francois, "04.05.93")?;
    reg_service.book_room(&room1, &jeanne, "20.10.02")?;
    reg_service.book_room(&room2, &jeanne, "20.10.02")?;
    reg_service.book_room(&room3, &francois, "05.12.25")?;

    user_service.remove_user("David")?;
    room_service.remove_room("Black room")?;

    println!("User's list: {:?}",user_service.list_users());
    println!("Is this user exists yet? {:?}",user_service.is_exist_user("David"));

    println!("Registery book {:#?}", reg_service.print_book());

    Ok(())
}
