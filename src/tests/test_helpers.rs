use crate::{
    domain::{Book, Room, User},
    error::{ErrDB, ErrDomain, ErrUser},
    infra::{
        in_memo_repo::{self, InMemoryRepo},
        reg_service::{self, RegService},
        room_service::{self, RoomService},
        user_service::{self, UserService},
    },
};

pub fn default_user1() -> Result<User, ErrDomain> {
    let user1 = User::new("Carmen Test")?;
    Ok(user1)
}

pub fn default_user2() -> Result<User, ErrDomain> {
    let user2 = User::new("Ibrahim Test")?;
    Ok(user2)
}

pub fn default_users() -> Result<(User, User), ErrDomain> {
    let user1 = User::new("Carmen Test")?;
    let user2 = User::new("Ibrahim Test")?;
    Ok((user1, user2))
}



pub fn default_rooms() -> Result<(Room, Room), ErrDomain> {
    let room1 = Room::new("Test room1")?;
    let room2 = Room::new("Test room2")?;
    Ok((room1, room2))
}

pub fn init_user_service() -> Result<UserService<InMemoryRepo<User>>, ErrDB> {
    Ok(UserService::new(InMemoryRepo::new()))
}

pub fn init_room_service() -> Result<RoomService<InMemoryRepo<Room>>, ErrDB> {
    Ok(RoomService::new(InMemoryRepo::new()))
}

pub fn init_reg_service() -> Result<RegService<InMemoryRepo<Book>>, ErrDB> {
    Ok(RegService::new(InMemoryRepo::new()))
}

pub fn init_inmemory_repo<T>() -> Result<InMemoryRepo<T>, ErrDB> {
    Ok(InMemoryRepo::new())
}
