use std::error::Error;

use crate::{domain::Room, repository::DBRepository};

pub struct RoomService<T> {
    repo: T,
}

impl<T> RoomService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T: DBRepository<Room>> RoomService<T> {
    pub fn add_room(&mut self, room: &str) -> Result<Room, Box<dyn Error>> {
        let room: Room = Room::new(room)?;
        self.repo.insert_data(&room)?;
        Ok(room)
    }

    pub fn remove_room(&mut self, room: &str) -> Result<(), Box<dyn Error>> {
        let room_list = self.repo.list()?;
        if let Some(data) = room_list.iter().find(|x| x.name.name == room) {
            return Ok(self.repo.remove_data(data)?);
        } else {
            Err("This room doesn't exist".into())
        }
    }

    pub fn list_rooms(&self) -> Result<Vec<Room>, Box<dyn Error>> {
        self.repo.list()
    }

    pub fn is_exist_room(&self, data: &str) -> Result<bool, Box<dyn Error>> {
        let room_list = self.repo.list()?;
        if room_list.iter().any(|x| x.name.name == data) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
