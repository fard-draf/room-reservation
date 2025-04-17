use crate::{
    domain::Room,
    error::{ErrRepo, ErrService},
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
    pub async fn add_room(&self, room: &str) -> Result<Room, ErrService> {
        let room: Room = Room::new(room)?;
        let room = self.repo.insert_room(&room).await?;
        Ok(room)
    }

    pub async fn delete_room_by_id(&mut self, room: i32) -> Result<(), ErrService> {
        let deleted = self.repo.delete_room_by_id(room).await?;
        if deleted {
            return Ok(());
        } else {
            Err(ErrService::Repo(ErrRepo::Unreachable))
        }
    }

    pub async fn list_rooms(&self) -> Result<Vec<Room>, ErrService> {
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
