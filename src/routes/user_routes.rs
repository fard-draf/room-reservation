use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    handlers::user_handlers::{SharedUserService, create_user, delete_user, list_users},
    infra::user_repo::UserRepo,
};

pub fn user_routes<T>(service: SharedUserService<T>) -> Router
where
    T: UserRepo + Send + Sync + 'static,
{
    Router::new()
        .route("/users", post(create_user::<T>))
        .route("/users", get(list_users::<T>))
        .route("/users", delete(delete_user::<T>))
        .with_state(service)
}
