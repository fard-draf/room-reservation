use serde::{Deserialize, Serialize};

use crate::{
    domain::{Room, RoomName},
    error::ErrDomain,
};

#[derive(Deserialize)]
pub struct CreateRoomDto {
    pub room_name: String,
}

#[derive(Deserialize)]
pub struct DeleteRoomByIdDto {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct UpdateRoomDto {
    pub old_name: String,
    pub new_name: String,
}

#[derive(Serialize)]
pub struct RoomDto {
    pub id: i32,
    pub room_name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RoomRowDto {
    pub id: i32,
    pub room_name: String,
}

impl TryFrom<CreateRoomDto> for Room {
    type Error = ErrDomain;

    fn try_from(dto: CreateRoomDto) -> Result<Self, Self::Error> {
        Ok(Room {
            id: 0,
            room_name: RoomName::new(&dto.room_name)?,
        })
    }
}

impl TryFrom<RoomRowDto> for Room {
    type Error = ErrDomain;

    fn try_from(dto: RoomRowDto) -> Result<Self, Self::Error> {
        Ok(Room {
            id: dto.id,
            room_name: RoomName::new(&dto.room_name)?,
        })
    }
}

impl From<Room> for RoomDto {
    fn from(room: Room) -> Self {
        RoomDto {
            id: room.id,
            room_name: room.room_name.name,
        }
    }
}
