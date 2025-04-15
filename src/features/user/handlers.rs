use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};


use std:: sync::Arc;
use tokio::sync::Mutex;

use crate::{
    app::state::AppState,
    features::user::{
        dto::{CreateUserDto, UserDto},
        repo::UserRepo,
        service::UserService
    },
    error::{ErrDB, ErrService},   
};

pub type SharedUserService<T> = Arc<Mutex<UserService<T>>>;

pub async fn create_user<T> (
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<impl IntoResponse, ErrService>
where 
    T: UserRepo + Send + 'static
    {
        let service = state.user_service.lock().await;

        let dto = service
            .add_user(&payload.user_name)
            .await?;
        let user_dto = UserDto {
            id: dto.id,
            user_name: dto.user_name.name,
        };

        Ok(Json(user_dto))
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ErrDB> 

{
    let service = state.user_service.lock().await;
    let users = service.list_users().await?;

    let dto: Vec<UserDto> = users
        .into_iter()
        .map(|u| UserDto {
            id: u.id,
            user_name: u.user_name.name,
        })
        .collect();

    Ok(Json(dto))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<impl IntoResponse, ErrService> 

{
    let mut service = state.user_service.lock().await;
    
    let dto = service
        .delete_user(&payload.user_name)
        .await?;


    Ok(())

}
