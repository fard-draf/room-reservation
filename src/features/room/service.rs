use super::repo::RoomRepo;
use crate::{
    domain::{Room, RoomName},
    error::{ErrRepo, ErrRoom, ErrService},
};

use dashmap::DashSet;
use tracing::{error, info, warn};

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
        info!(
            "Room added to cache: {:?} cache has now {} entries",
            room,
            self.cache.len()
        );
        Ok(room)
    }

    pub async fn update_room(&self, old_room: &str, new_room: &str) -> Result<Room, ErrService> {
        let old_room = Room::new(old_room)?;
        let new_room = Room::new(new_room)?;

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
                error!("Failed to update room: {:?}", e);
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
                warn!(
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
        Ok(self.cache.iter().map(|r| r.key().clone()).collect())
    }

    pub async fn is_exist_room(&self, room_name: &RoomName) -> Result<bool, ErrService> {
        Ok(self.cache.iter().any(|r| r.room_name == *room_name))
    }

    pub async fn get_cache_room_by_room_struct(&self, room: &Room) -> Result<Room, ErrService> {
        self.cache
            .get(room)
            .map(|value| {
                info!("room founded: {:?}", value.room_name);
                value.clone()
            })
            .ok_or_else(|| ErrService::Room(ErrRoom::RoomNotFound))
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
        rooms.into_iter().for_each(|e| {
            let room = Room {
                id: e.id,
                room_name: e.room_name,
            };
            self.cache.insert(room);
        });

        info!("RoomService cache lenght: {}", self.cache.len());
        Ok(())
    }
}
