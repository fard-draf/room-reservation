use std::error::Error;

use domain::{Book, Room, User};
use infra::{
    in_memo_repo::InMemoryRepo,
    user_service::UserService,
    room_service::RoomService,
    reg_service::RegService,
};

mod domain;
mod infra;
mod repository;
mod run;

fn main() -> Result<(), Box<dyn Error>> {
    
    

    let repo_room: InMemoryRepo<Room> = InMemoryRepo::new();
    let mut room_service: RoomService<InMemoryRepo<Room>> = RoomService::new(repo_room);

    let repo_reg: InMemoryRepo<Book> = InMemoryRepo::new();
    let mut reg_service: RegService<InMemoryRepo<Book>> = RegService::new(repo_reg);

    



    // println!(" User's list: {:#?}",user_service.list_users());

    // let room1 = room_service.add_room("Suite Royale")?;
    // let room2 = room_service.add_room("Etape noctrne")?;
    // let room3 = room_service.add_room("Dolce Note")?;
    // let room4 = room_service.add_room("Black room")?;
    
    // println!("Is this room exist? {:#?}",room_service.is_exist_room("El camino"));
    // println!("Is this room exist? {:#?}",room_service.is_exist_room("Dolce Note"));
    // println!("Room's list: {:#?}",room_service.list_rooms());
    

    // reg_service.book_room(&room1, &francois, "20.10.02")?;
    // reg_service.book_room(&room2, &francois, "04.05.93")?;
    // reg_service.book_room(&room1, &jeanne, "20.10.02")?;
    // reg_service.book_room(&room2, &jeanne, "20.10.02")?;
    // reg_service.book_room(&room3, &francois, "05.12.25")?;


    // user_service.remove_user(&david)?;
    // room_service.remove_room(&room4)?;

    // println!("User's list: {:?}",user_service.list_users());
    // println!("Is this user exists yet? {:?}",user_service.is_exist_user(&david));


    // println!("Registery book {:#?}", reg_service.print_book());


    Ok(())
}
