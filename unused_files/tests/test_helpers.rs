use crate::{
    domain::{Book, Room, User},
    error::ErrDomain,
    tests::in_memory_repo::InMemoryRepo,
};

pub async fn default_user1() -> Result<User, ErrDomain> {
    Ok(User::new("Carmen Test")?)
}

pub async fn default_user2() -> Result<User, ErrDomain> {
    Ok(User::new("Ibrahim Test")?)
}

pub async fn default_users() -> Result<(User, User), ErrDomain> {
    Ok((User::new("Carmen Test")?, User::new("Ibrahim Test")?))
}

pub async fn default_rooms() -> Result<(Room, Room), ErrDomain> {
    Ok((Room::new("Test room1")?, Room::new("Test room2")?))
}

pub async fn init_user_service() -> Result<InMemoryRepo<User>, ErrDomain> {
    Ok(InMemoryRepo::<User>::new())
}

pub async fn init_inmemory_repo<T>() -> Result<InMemoryRepo<T>, ErrDomain> {
    Ok(InMemoryRepo::<T>::new())
}
