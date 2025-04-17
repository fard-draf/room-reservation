use axum::{Json, extract::State, response::IntoResponse};

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    app::state::AppState,
    error::ErrService,
    features::room::{
        dto::{CreateRoomDto, RoomDto},
        service::RoomService,
    },
};

use super::dto::DeleteRoomByIdDto;

pub type SharedRoomService<T> = Arc<Mutex<RoomService<T>>>;

pub async fn create_room(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoomDto>,
) -> Result<impl IntoResponse, ErrService> {
    let service = state.room_service.lock().await;

    let dto = service.add_room(&payload.room_name).await?;
    let room_dto = RoomDto {
        id: dto.id,
        room_name: dto.room_name.name,
    };

    Ok(Json(room_dto))
}

pub async fn list_room(State(state): State<AppState>) -> Result<impl IntoResponse, ErrService> {
    let service = state.room_service.lock().await;
    let rooms = service.list_rooms().await?;

    let dto: Vec<RoomDto> = rooms
        .into_iter()
        .map(|u| RoomDto {
            id: u.id,
            room_name: u.room_name.name,
        })
        .collect();

    Ok(Json(dto))
}

pub async fn delete_room(
    State(state): State<AppState>,
    Json(payload): Json<DeleteRoomByIdDto>,
) -> Result<impl IntoResponse, ErrService> {
    let mut service = state.room_service.lock().await;

    service.delete_room_by_id(payload.id).await?;

    Ok(())
}
