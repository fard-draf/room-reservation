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
    domain::User, dto::user_dto::{CreateUserDto, DeleteUserDto, UserDto}, error::{AppError, ErrService}, infra::user_service::UserService, repository::DBRepository
};

pub type SharedUserService<T> = Arc<Mutex<UserService<T>>>;

pub async fn create_user<T> (
    State(service): State<SharedUserService<T>>,
    Json(payload): Json<CreateUserDto>,
) -> Result<impl IntoResponse, AppError>
where 
    T: DBRepository<User> + Send + 'static
    {
        let mut service = service.lock().await;

        match service.add_new_user(&payload.name).await {
            Ok(user) => {
                let user_dto = UserDto {
                    name: user.name.name,
                };
                Ok((StatusCode::CREATED, Json(serde_json::to_value(user_dto).unwrap())))
            }
            Err(ErrService::UserCreation(_)) => {
                Ok((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "error": "Invalid user data"
                }))))
            }
            Err(_) => {
                Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "error": "Failed to create user" 
                }))))
            }
        }

}

pub async fn list_users<T>(
    State(service): State<SharedUserService<T>>,
) -> Result<impl IntoResponse, AppError> 
where
    T: DBRepository<User> + Send + 'static,
{
    let service = service.lock().await;
    
    match service.list_users().await {
        Ok(users) => {
            let users_dto: Vec<UserDto> = users
                .into_iter()
                .map(|user| UserDto { name: user.name.name })
                .collect();
            Ok((StatusCode::OK, Json(serde_json::to_value(users_dto).unwrap())))
        }
        Err(_) => {
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to list users"
            }))))
        }
    }
}

pub async fn delete_user<T>(
    State(service): State<SharedUserService<T>>,
    Json(payload): Json<DeleteUserDto>,
) -> Result<impl IntoResponse, AppError> 
where 
    T: DBRepository<User> + Send + 'static,
{
    let mut service = service.lock().await;

    match service.remove_user(&payload.name).await {
        Ok(user) => Ok((StatusCode::OK.into_response())),
        
        Err(ErrService::UserCreation(_)) => {
            let body = serde_json::json!({"error" : "Invalid user data"});
            Ok((StatusCode::BAD_REQUEST, Json(body)).into_response())
        }
        Err(_) => {
            let body = serde_json::json!({"error" : "Failed to delete user"});
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response())
        }
    }
}
