#[cfg(test)]
mod test {

    use crate::{
        domain::{Room, RoomName},
        error::{ErrRepo, ErrRoom, ErrService},
        features::room::{repo::RoomRepo, service::RoomService},
        infra::in_memory::in_memo_repo::InMemoryRepo,
    };

    use async_trait::async_trait;

    #[async_trait]
    impl RoomRepo for InMemoryRepo<Room> {
        async fn insert_room(&self, room: &Room) -> Result<Room, ErrService> {
            self.repo.write().await.insert(room.clone());
            Ok(room.clone())
        }
        async fn update_room(&self, id: i32, new_name: RoomName) -> Result<Room, ErrService> {
            let old_room = {
                let read_guard = self.repo.read().await;
                let old_room = read_guard.iter().find(|r| r.id == id).cloned();
                old_room
            };

            let mut write_guard = self.repo.write().await;
            if let Some(old_room) = old_room {
                let new_room = Room {
                    room_name: new_name,
                    id: old_room.id,
                };
                write_guard.remove(&old_room);
                write_guard.insert(new_room.clone());
                Ok(new_room)
            } else {
                Err(ErrService::Repo(ErrRepo::Unreachable))
            }
        }

        async fn delete_room_by_id(&self, room_id: i32) -> Result<bool, ErrService> {
            let room = {
                let read_guard = self.repo.read().await;
                let room = read_guard.iter().find(|r| r.id == room_id).cloned();
                room
            };

            let mut write_guard = self.repo.write().await;
            if let Some(room) = room {
                write_guard.remove(&room);
            } else {
                return Err(ErrService::Room(ErrRoom::RoomNotFound));
            }
            Ok(true)
        }
        async fn get_all_rooms(&self) -> Result<Vec<Room>, ErrService> {
            Ok(self.repo.read().await.iter().cloned().collect())
        }
        async fn get_one_room(&self, room_name: &RoomName) -> Result<Room, ErrService> {
            if let Some(room) = self
                .repo
                .read()
                .await
                .iter()
                .find(|r| r.room_name == *room_name)
            {
                Ok(room.clone())
            } else {
                Err(ErrService::Room(ErrRoom::RoomNotFound))
            }
        }
    }

    #[tokio::test]
    async fn add_and_list_rooms() {
        let service: RoomService<InMemoryRepo<Room>> = InMemoryRepo::init_room_service().await;

        let room1 = service.add_room("room1").await.unwrap();

        assert!(service.add_room("room1").await.is_err()); // already exists
        assert!(service.add_room("room2").await.is_ok());
        assert!(service.add_room("r").await.is_err()); // too short
        assert!(
            service
                .add_room("thisnameistoolongforasingleroom")
                .await
                .is_err()
        );
        assert!(service.list_cache_rooms().await.is_ok());
        assert!(service.is_exist_room(&room1.room_name).await.is_ok());

        assert!(service.get_cache_room_by_room_struct(&room1).await.is_ok());
    }
}
