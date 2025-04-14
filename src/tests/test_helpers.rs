use crate::{
    domain::{Book, Room, User},
    error::{ErrDB, ErrDomain},
    infra::{
        in_memo_repo::InMemoryRepo,
        reg_service::RegService,
        room_service::RoomService,
        user_service::UserService,
    },
};

pub fn default_user1() -> Result<User, ErrDomain> {
    Ok(User::new("Carmen Test")?)
}

pub fn default_user2() -> Result<User, ErrDomain> {
    Ok(User::new("Ibrahim Test")?)
}

pub fn default_users() -> Result<(User, User), ErrDomain> {
    Ok((User::new("Carmen Test")?, User::new("Ibrahim Test")?))
}

pub fn default_rooms() -> Result<(Room, Room), ErrDomain> {
    Ok((Room::new("Test room1")?, Room::new("Test room2")?))
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
