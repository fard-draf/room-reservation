use super::repo::RoomRepo;
use crate::{
    domain::{Room, RoomName},
    error::{ErrRepo, ErrRoom, ErrService},
};

use dashmap::DashSet;
use tracing::info;

#[derive(Debug)]
pub struct RoomService<T> {
    repo: T,
    cache: DashSet<Room>,
}

impl<T> RoomService<T> {
    pub fn new(repo: T) -> Self {
        Self {
            repo,
            cache: DashSet::new(),
        }
    }
}

impl<T: RoomRepo> RoomService<T> {
    pub async fn add_room(&self, room: &str) -> Result<Room, ErrService> {
        let room: Room = Room::new(room)?;
        info!("cache lenght : {}", self.cache.len());

        if self.is_exist_room(&room.room_name).await? {
            return Err(ErrService::Room(ErrRoom::AlreadyExist));
        }

        let room = self.repo.insert_room(&room).await?;
        self.cache.insert(room.clone());
        tracing::info!(
            "Room added to cache: {:?} cache has now {} entries",
            room,
            self.cache.len()
        );
        Ok(room)
    }

    pub async fn update_room(&self, old_room: &str, new_room: &str) -> Result<Room, ErrService> {
        let old_room = Room::new(old_room)?;
        let new_room = Room::new(new_room)?;
        info!("cache length : {}", self.cache.len());

        let cache_snapshot: Vec<_> = self.cache.iter().map(|r| r.clone()).collect();

        let o_room = cache_snapshot
            .iter()
            .find(|r| r.room_name == old_room.room_name)
            .ok_or(ErrService::Room(ErrRoom::RoomNotFound))?;

        if cache_snapshot
            .iter()
            .any(|r| r.room_name == new_room.room_name)
        {
            return Err(ErrService::Room(ErrRoom::AlreadyExist));
        }

        let room = self
            .repo
            .update_room(o_room.id, new_room.room_name.clone())
            .await
            .map_err(|e| {
                tracing::error!("Failed to update room: {:?}", e);
                ErrService::Repo(ErrRepo::BadRequest)
            })?;

        self.cache.remove(o_room);
        self.cache.insert(Room {
            id: o_room.id,
            room_name: new_room.room_name,
        });

        Ok(room)
    }

    pub async fn delete_room_by_id(&self, room: i32) -> Result<(), ErrService> {
        let deleted = self.repo.delete_room_by_id(room).await?;

        if deleted {
            if let Some(room_founded) = self.get_room_by_id_on_cache(room)? {
                self.cache.remove(&room_founded);
            } else {
                tracing::warn!(
                    "Deleted room from database, but not found in cache: id = {}",
                    room
                );
            }
            Ok(())
        } else {
            Err(ErrService::Room(ErrRoom::RoomNotFound))
        }
    }

    pub async fn list_rooms(&self) -> Result<Vec<Room>, ErrService> {
        self.repo.get_all_rooms().await
    }

    pub async fn list_cache_rooms(&self) -> Result<Vec<Room>, ErrService> {
        let vec: Vec<Room> = self.cache.iter().map(|r| r.key().clone()).collect();
        Ok(vec)
    }

    pub async fn is_exist_room(&self, room_name: &RoomName) -> Result<bool, ErrService> {
        Ok(self.cache.iter().any(|r| r.room_name == *room_name))
    }

    pub async fn get_cache_room_by_room_struct(&self, room: &Room) -> Result<Room, ErrService> {
        if let Some(value) = self.cache.get(room) {
            info!("room founded: {:?}", value.room_name);
            Ok(value.clone())
        } else {
            info!("room not founded");
            Err(ErrService::Room(ErrRoom::RoomNotFound))
        }
    }

    pub fn get_room_by_id_on_cache(&self, room_id: i32) -> Result<Option<Room>, ErrService> {
        Ok(self
            .cache
            .iter()
            .find(|x| x.id == room_id)
            .map(|r| r.clone()))
    }

    pub async fn populate_cache(&self) -> Result<(), ErrService> {
        let rooms = self.repo.get_all_rooms().await?;
        for element in rooms {
            let room = Room {
                id: element.id,
                room_name: element.room_name,
            };
            self.cache.insert(room);
        }
        info!(
            "[populate_cache] instance RoomService: {}",
            self.cache.len()
        );
        Ok(())
    }
}
