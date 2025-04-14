use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tower::Service;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    dto::user_dto::{CreateUserDto, UserDto},
    error::{ErrDB, ErrService},
    infra::user_service::UserService,
    domain::User,
    repository::DBRepository,
};

pub type SharedUserService<T> = Arc<Mutex<UserService<T>>>;

pub async fn create_user<T> (
    State(service): State<SharedUserService<T>>,
    Json(payload): Json<CreateUserDto>,
) -> impl IntoResponse 
where 
    T: DBRepository<User> + Send + 'static
    {
        let mut service = service.lock().await;

        match service.add_new_user(&payload.name) {
            Ok(user) => {
                let user_dto = UserDto {
                    name: user.name.name,
                };
                (StatusCode::CREATED, Json(serde_json::to_value(user_dto).unwrap()))
            }
            Err(ErrService::UserCreation(_)) => {
                (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Invalid user data"
                })))
            }
            Err(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "error": "Failed to create user" 
                })))
            }
        }

}

pub async fn list_users<T>(
    State(service): State<SharedUserService<T>>,
) -> impl IntoResponse 
where
    T: DBRepository<User> + Send + 'static,
{
    let service = service.lock().await;
    
    match service.list_users() {
        Ok(users) => {
            let users_dto: Vec<UserDto> = users
                .into_iter()
                .map(|user| UserDto { name: user.name.name })
                .collect();
            (StatusCode::OK, Json(serde_json::to_value(users_dto).unwrap()))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to list users"
            })))
        }
    }
}