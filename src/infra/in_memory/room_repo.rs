use std::{collections::HashSet, hash::Hash};
use tokio::sync::RwLock;

use crate::features::{
    book::service::BookService, room::service::RoomService, user::service::UserService,
};

#[derive(Debug)]
pub struct InMemoryRepo<T> {
    pub repo: RwLock<HashSet<T>>,
    pub cache: HashSet<T>,
}

impl<T: Eq + Hash + Clone> InMemoryRepo<T> {
    pub async fn new() -> Self {
        Self {
            repo: RwLock::new(HashSet::<T>::new()),
            cache: HashSet::new(),
        }
    }

    pub async fn init_user_service() -> UserService<InMemoryRepo<T>> {
        let repo: InMemoryRepo<T> = InMemoryRepo::new().await;
        let service: UserService<InMemoryRepo<T>> = UserService::new(repo);
        service
    }

    pub async fn init_room_service() -> RoomService<InMemoryRepo<T>> {
        let repo: InMemoryRepo<T> = InMemoryRepo::new().await;
        let service: RoomService<InMemoryRepo<T>> = RoomService::new(repo);
        service
    }
    pub async fn init_book_service() -> BookService<InMemoryRepo<T>> {
        let repo: InMemoryRepo<T> = InMemoryRepo::new().await;
        let service: BookService<InMemoryRepo<T>> = BookService::new(repo);
        service
    }
}
