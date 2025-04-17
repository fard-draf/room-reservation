use crate::{
    domain::Room,
    error::{ErrRepo, ErrRoom, ErrService},
    features::room::dto::RoomRowDto,
    infra::db::DBClient,
};

use async_trait::async_trait;

#[async_trait]
pub trait RoomRepo {
    async fn insert_room(&self, room: &Room) -> Result<Room, ErrService>;
    async fn delete_room_by_id(&self, room: i32) -> Result<bool, ErrService>;
    async fn get_all_rooms(&self) -> Result<Vec<Room>, ErrService>;
}

#[async_trait]
impl RoomRepo for DBClient {
    async fn insert_room(&self, room: &Room) -> Result<Room, ErrService> {
        
        let mut existing_room = vec![];
        for element in self.get_all_rooms().await? {
            existing_room.push(element.room_name);
        }
        if existing_room.contains(&room.room_name) {
            return Err(ErrService::Room(ErrRoom::AlreadyExist));
        }        
        let row = sqlx::query_as::<_, RoomRowDto>(
            "INSERT INTO rooms (room_name) VALUES ($1) RETURNING id, room_name",
        )
        .bind(room.room_name.name.clone())
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| ErrRepo::Unreachable)?;

        let room: Room = row.try_into()?;
        Ok(room)
    }

    async fn delete_room_by_id(&self, room_name: i32) -> Result<bool, ErrService> {
        let result = sqlx::query("DELETE FROM rooms WHERE id = $1")
            .bind(room_name)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrRepo::DoesntExist)?;

        if result.rows_affected() == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    async fn get_all_rooms(&self) -> Result<Vec<Room>, ErrService> {
        let vec = sqlx::query_as::<_, RoomRowDto>("SELECT id, room_name FROM rooms")
            .fetch_all(&self.pool)
            .await
            .map_err(|_e| ErrRepo::BadRequest)?;
        let rooms = vec
            .into_iter()
            .map(|dto| dto.try_into().map_err(|_| ErrRepo::DoesntExist))
            .collect::<Result<_, _>>()?;
        Ok(rooms)
    }
}
