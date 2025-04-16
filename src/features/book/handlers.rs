use axum::{Json, extract::State, response::IntoResponse};

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    app::state::AppState,
    domain::{Book, BookDate, Room, RoomName, User, UserName},
    error::{ErrDB, ErrService},
    features::{
        book::{
            dto::{BookDto, CreateBookDto},
            repo::BookRepo,
            service::BookService,
        },
        room,
    },
};

use super::dto::{self, DeleteBookByIdDto};

pub type SharedRoomService<T> = Arc<Mutex<BookService<T>>>;

pub async fn create_booking(
    State(state): State<AppState>,
    Json(payload): Json<CreateBookDto>,
) -> Result<impl IntoResponse, ErrService> {
    let mut service = state.book_service.lock().await;

    let dto = service
        .book_room(&payload.room_name, &payload.user_name, &payload.date)
        .await?;
    let book_dto = BookDto {
        id: 0,
        room_name: dto.room_name.name,
        user_name: dto.user_name.name,
        date: dto.date.date.to_string(),
    };

    Ok(Json(book_dto))
}

pub async fn list_book(State(state): State<AppState>) -> Result<impl IntoResponse, ErrDB> {
    let service = state.book_service.lock().await;
    let rooms = service.list_book().await?;

    let dto: Vec<BookDto> = rooms
        .into_iter()
        .map(|u| BookDto {
            id: u.id,
            user_name: u.user_name.name,
            room_name: u.room_name.name,
            date: u.date.date.to_string(),
        })
        .collect();

    Ok(Json(dto))
}

pub async fn delete_book(
    State(state): State<AppState>,
    Json(payload): Json<DeleteBookByIdDto>,
) -> Result<impl IntoResponse, ErrService> {
    let mut service = state.book_service.lock().await;

    service.delete_book_by_id(payload.id).await?;

    Ok(())
}
