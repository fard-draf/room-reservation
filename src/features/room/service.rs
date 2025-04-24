use std::collections::HashSet;
use tokio::sync::RwLock;

use super::repo::RoomRepo;
use crate::{
    domain::{Room, RoomName},
    error::{ErrRepo, ErrRoom, ErrService},
};

#[derive(Debug)]
pub struct RoomService<T> {
    pub repo: T,
    cache: RwLock<HashSet<Room>>,
}

impl<T> RoomService<T> {
    pub fn new(repo: T) -> Self {
        Self {
            repo,
            cache: RwLock::new(HashSet::new()),
        }
    }
}

impl<T: RoomRepo> RoomService<T> {
    pub async fn add_room(&self, room: &str) -> Result<Room, ErrService> {
        println!("[add_room] instance RoomService: {:p}", self);
        let room: Room = Room::new(room)?;

        let is_existing_room = self.is_exist_room(&room.room_name).await?;

        if is_existing_room {
            return Err(ErrService::Room(ErrRoom::AlreadyExist));
        }

        let room = self.repo.insert_room(&room).await?;
        let mut cache = self.cache.write().await;
        cache.insert(room.clone());
        tracing::info!(
            "Room added to cache: {:?} cache has now {} entries",
            room,
            cache.len()
        );
        Ok(room)
    }

    pub async fn update_room(&self, old_room: &str, new_room: &str) -> Result<Room, ErrService> {
        let old_room = Room::new(old_room)?;
        let new_room = Room::new(new_room)?;

        if self.cache.read().await.contains(&new_room) {
            return Err(ErrService::Room(ErrRoom::AlreadyExist));
        }

        if !self.cache.read().await.contains(&old_room) {
            return Err(ErrService::Room(ErrRoom::RoomNotFound));
        }

        let rooms = self.get_cache_room_by_room_struct(&old_room).await?;

        let room = self.repo.update_room(rooms.id, new_room.room_name).await?;

        Ok(room)
    }

    pub async fn delete_room_by_id(&self, room: i32) -> Result<(), ErrService> {
        let deleted = self.repo.delete_room_by_id(room).await?;
        if deleted {
            let id = self
                .get_cache_room_by_id(room)
                .await?
                .ok_or(ErrService::Room(ErrRoom::RoomNotFound))?;
            self.cache.write().await.remove(&id);
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::UnableToDelete))
        }
    }

    pub async fn list_rooms(&self) -> Result<Vec<Room>, ErrService> {
        self.repo.get_all_rooms().await
    }

    pub async fn list_cache_rooms(&self) -> Result<Vec<Room>, ErrService> {
        let mut vec: Vec<Room> = Vec::new();
        for element in self.cache.read().await.iter() {
            vec.push(element.clone());
        }
        Ok(vec)
    }

    pub async fn is_exist_room(&self, data: &RoomName) -> Result<bool, ErrService> {
        let room_list = self.cache.read().await;
        if room_list.iter().any(|x| x.room_name == *data) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn get_cache_room_by_room_struct(&self, room: &Room) -> Result<Room, ErrService> {
        if let Some(value) = self.cache.read().await.get(&room.clone()) {
            Ok(value.clone())
        } else {
            Err(ErrService::Room(ErrRoom::RoomNotFound))
        }
    }

    pub async fn get_cache_room_by_id(&self, room_id: i32) -> Result<Option<Room>, ErrService> {
        if let Some(room_by_id) = self.cache.read().await.iter().find(|x| x.id == room_id) {
            Ok(Some(room_by_id.clone()))
        } else {
            Ok(None)
        }
    }

    pub async fn populate_cache(&self) -> Result<(), ErrService> {
        println!("[populate_cache] instance RoomService: {:p}", self);
        let rooms = self.repo.get_all_rooms().await?;
        for element in rooms {
            let room = Room {
                id: element.id,
                room_name: element.room_name,
            };
            self.cache.write().await.insert(room);
        }
        Ok(())
    }
}
