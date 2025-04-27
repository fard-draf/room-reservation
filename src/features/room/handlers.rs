use axum::{Json, extract::State, response::IntoResponse};
use tracing::info;

use std::sync::Arc;

use crate::{
    app::state::AppState,
    error::ErrService,
    features::room::{
        dto::{CreateRoomDto, RoomDto},
        service::RoomService,
    },
};

use super::dto::{DeleteRoomByIdDto, UpdateRoomDto, UpdateRoomNameDto};

pub type SharedRoomService<T> = Arc<RoomService<T>>;

pub async fn create_room(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoomDto>,
) -> Result<impl IntoResponse, ErrService> {
    info!("create room tag");
    let service = state.room_service;
    info!("create room tag after service");

    let dto = service.add_room(&payload.room_name).await?;
    info!("create room tag after dto");

    let room_dto = RoomDto {
        id: dto.id,
        room_name: dto.room_name.name,
    };

    Ok(Json(room_dto))
}

pub async fn update_room_name(
    State(state): State<AppState>,
    Json(payload): Json<UpdateRoomNameDto>,
) -> Result<impl IntoResponse, ErrService> {
    let service = state.room_service;

    let dto = service
        .update_room(&payload.old_name, &payload.new_name)
        .await?;

    let room_dto = UpdateRoomDto {
        id: dto.id,
        new_name: dto.room_name.name,
    };

    Ok(Json(room_dto))
}

pub async fn list_room(State(state): State<AppState>) -> Result<impl IntoResponse, ErrService> {
    let service = state.room_service;
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
    let service = state.room_service;

    service.delete_room_by_id(payload.id).await?;

    Ok(())
}
