use crate::{
    domain::Room,
    error::{ErrDB, ErrService},
};

use super::repo::RoomRepo;

pub struct RoomService<T> {
    repo: T,
}

impl<T> RoomService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T: RoomRepo> RoomService<T> {
    pub async fn add_room(&mut self, room: &str) -> Result<Room, ErrService> {
        let room: Room = Room::new(room)?;
        self.repo.insert_room(&room).await?;
        Ok(room)
    }

    pub async fn delete_room(&mut self, room: &str) -> Result<(), ErrService> {
        let room_list = self.repo.get_all_rooms().await?;
        if let Some(data) = room_list.iter().find(|x| x.room_name.name == room) {
            return Ok(self.repo.delete_room_by_name(data).await?);
        } else {
            Err(ErrService::DbRequest(ErrDB::Unreachable))
        }
    }

    pub async fn list_rooms(&self) -> Result<Vec<Room>, ErrDB> {
        self.repo.get_all_rooms().await
    }

    pub async fn is_exist_room(&self, data: &str) -> Result<bool, ErrService> {
        let room_list = self.repo.get_all_rooms().await?;
        if room_list.iter().any(|x| x.room_name.name == data) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
