use crate::{domain::Room, error::ErrDB, features::room::dto::RoomRowDto, infra::db::DBClient};

use async_trait::async_trait;

#[async_trait]
pub trait RoomRepo {
    async fn insert_room(&self, room: &Room) -> Result<(), ErrDB>;
    async fn delete_room_by_id(&self, room: i32) -> Result<(), ErrDB>;
    async fn get_all_rooms(&self) -> Result<Vec<Room>, ErrDB>;
}

#[async_trait]
impl RoomRepo for DBClient {
    async fn insert_room(&self, room: &Room) -> Result<(), ErrDB> {
        sqlx::query("INSERT INTO rooms (id, name) VALUES ($1, $2)")
            .bind(room.id)
            .bind(room.room_name.name.clone())
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrDB::Unreachable)?;
        Ok(())
    }

    async fn delete_room_by_id(&self, room_name: i32) -> Result<(), ErrDB> {
        sqlx::query("DELETE FROM rooms WHERE id = $1")
            .bind(room_name)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrDB::DoesntExist)?;
        Ok(())
    }

    async fn get_all_rooms(&self) -> Result<Vec<Room>, ErrDB> {
        let vec = sqlx::query_as::<_, RoomRowDto>("SELECT id, name FROM rooms")
            .fetch_all(&self.pool)
            .await
            .map_err(|_e| ErrDB::Unreachable)?;
        let rooms = vec
            .into_iter()
            .map(|dto| dto.try_into().map_err(|_| ErrDB::DoesntExist))
            .collect::<Result<_, _>>()?;
        Ok(rooms)
    }
}
