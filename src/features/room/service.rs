use crate::{
    domain::{Room, RoomName},
    error::{ErrRepo, ErrRoom, ErrService},
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

    pub async fn update_room(&self, old_name: &str, new_name: &str) -> Result<Room, ErrService> {
        let old_name = RoomName::new(old_name)?;
        let new_name = RoomName::new(new_name)?;

        let rooms = self.repo.get_all_rooms().await?;

        let existing_room = rooms
            .iter()
            .find(|r| r.room_name == old_name)
            .cloned()
            .ok_or(ErrService::Room(ErrRoom::RoomNotFound))?;

        if rooms.iter().any(|r| r.room_name == new_name) {
            return Err(ErrService::Room(ErrRoom::AlreadyExist));
        }

        let room = self.repo.update_room(existing_room.id, new_name).await?;

        Ok(room)
    }

    pub async fn delete_room_by_id(&mut self, room: i32) -> Result<(), ErrService> {
        let deleted = self.repo.delete_room_by_id(room).await?;
        if deleted {
            Ok(())
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
