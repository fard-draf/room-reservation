use axum::{
    routing::{get, post, delete},
    Router
};

use crate::{
    domain::User, handlers::user_handlers::{create_user, delete_user, list_users, SharedUserService}, repository::DBRepository
};

pub fn user_routes<T>(service: SharedUserService<T>) -> Router 
where 
    T: DBRepository<User> + Send + Sync + 'static
{
    Router::new()
        .route("/users", post(create_user::<T>))
        .route("/users", get(list_users::<T>))
        .route("/users", delete(delete_user::<T>))
        .with_state(service)

}
