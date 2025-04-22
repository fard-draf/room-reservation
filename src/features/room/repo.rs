use crate::{
    domain::{Room, RoomName},
    error::{ErrRepo, ErrRoom, ErrService},
    features::room::dto::RoomRowDto,
    infra::db::DBClient,
};

use async_trait::async_trait;

#[async_trait]
pub trait RoomRepo {
    async fn insert_room(&self, room: &Room) -> Result<Room, ErrService>;
    async fn update_room(&self, id: i32, new_name: RoomName) -> Result<Room, ErrService>;
    async fn delete_room_by_id(&self, room: i32) -> Result<bool, ErrService>;
    async fn get_all_rooms(&self) -> Result<Vec<Room>, ErrService>;
}

#[async_trait]
impl RoomRepo for DBClient {
    async fn insert_room(&self, room: &Room) -> Result<Room, ErrService> {
        if self
            .get_all_rooms()
            .await?
            .iter()
            .any(|r| r.room_name == room.room_name)
        {
            return Err(ErrService::Room(ErrRoom::AlreadyExist));
        }

        let row: RoomRowDto = sqlx::query_as::<_, RoomRowDto>(
            "INSERT INTO rooms (room_name) VALUES ($1) RETURNING id, room_name",
        )
        .bind(room.room_name.name.clone())
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| ErrRepo::Unreachable)?;

        let room: Room = row.try_into()?;
        Ok(room)
    }

    async fn update_room(&self, id: i32, new_name: RoomName) -> Result<Room, ErrService> {
        let row = sqlx::query_as::<_, RoomRowDto>(
            "UPDATE rooms SET room_name = $1 WHERE id = $2 RETURNING id, room_name",
        )
        .bind(new_name.name)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| ErrRepo::BadRequest)?;

        let room = Room {
            id: row.id,
            room_name: RoomName::new(&row.room_name)?,
        };

        Ok(room)
    }

    async fn delete_room_by_id(&self, room_name: i32) -> Result<bool, ErrService> {
        let result = sqlx::query("DELETE FROM rooms WHERE id = $1")
            .bind(room_name)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrRepo::DoesntExist)?;

        Ok(result.rows_affected() != 0)
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
