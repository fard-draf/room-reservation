use axum::{Json, extract::State, response::IntoResponse};

use std::sync::Arc;

use crate::{
    app::state::AppState,
    error::ErrService,
    features::user::{
        dto::{CreateUserDto, UpdateUserDto, UserDto},
        service::UserService,
    },
};

use super::dto::UpdateUserNameDto;

pub type SharedUserService<T> = Arc<UserService<T>>;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<impl IntoResponse, ErrService> {
    let service = state.user_service;

    let dto = service.add_user(&payload.user_name).await?;

    let user_dto = UserDto {
        user_id: dto.user_id.id,
        user_name: dto.user_name.name,
    };

    Ok(Json(user_dto))
}

pub async fn update_user(
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserNameDto>,
) -> Result<impl IntoResponse, ErrService> {
    let service = state.user_service;

    let dto = service
        .update_user(&payload.old_name, &payload.new_name)
        .await?;

    let user_dto = UpdateUserDto {
        user_id: dto.user_id.id,
        new_name: dto.user_name.name,
    };

    Ok(Json(user_dto))
}

pub async fn list_users(State(state): State<AppState>) -> Result<impl IntoResponse, ErrService> {
    let service = state.user_service;
    let users = service.list_users().await?;

    let dto: Vec<UserDto> = users
        .into_iter()
        .map(|u| UserDto {
            user_id: u.user_id.id,
            user_name: u.user_name.name,
        })
        .collect();

    Ok(Json(dto))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<impl IntoResponse, ErrService> {
    let service = state.user_service;

    service.delete_user_by_name(&payload.user_name).await?;

    Ok(())
}
