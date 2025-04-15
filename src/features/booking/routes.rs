use axum::{
    routing::{delete, get, post}, Router
 };
 
 use crate::{
     features::booking::
         handlers::{
             create_room,
             delete_room,
             list_room
             },
         app::state::AppState,
 };
 
 pub fn user_routes() -> Router<AppState> {
     Router::new()
         .route("/users", post(create_room))
         .route("/users", get(list_room))
         .route("/users", delete(delete_room))
 }