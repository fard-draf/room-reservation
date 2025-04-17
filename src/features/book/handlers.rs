use axum::{Json, extract::State, response::IntoResponse};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    app::state::AppState,
    error::ErrService,
    features::book::{
        dto::{BookDto, CreateBookDto},
        service::BookService,
    },
};

use super::dto::DeleteBookByIdDto;

pub type SharedBookService<T> = Arc<Mutex<BookService<T>>>;

pub async fn create_booking(
    State(state): State<AppState>,
    Json(payload): Json<CreateBookDto>,
) -> Result<impl IntoResponse, ErrService> {
    let service = state.book_service.lock().await;

    let dto = service
        .book_room(&payload.room_name, &payload.user_name, &payload.date)
        .await?;

    let book_dto = BookDto {
        id: dto.id,
        room_name: dto.room_name.name,
        user_name: dto.user_name.name,
        date: dto.date.date,
    };

    Ok(Json(book_dto))
}

pub async fn list_book(State(state): State<AppState>) -> Result<impl IntoResponse, ErrService> {
    let service = state.book_service.lock().await;
    let books = service.list_book().await.map_err(|e| {
        eprintln!("List book error: {:?}", e);
        e
    })?;
    let dto: Vec<BookDto> = books
        .into_iter()
        .map(|u| BookDto {
            id: u.id,
            user_name: u.user_name.name,
            room_name: u.room_name.name,
            date: u.date.date,
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

pub async fn delete_all_books(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ErrService> {
    let mut service = state.book_service.lock().await;

    service.delete_all_book().await?;

    Ok(())
}
