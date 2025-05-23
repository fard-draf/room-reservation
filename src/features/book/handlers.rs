use axum::{Json, extract::State, response::IntoResponse};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tracing::info;

use crate::{
    app::state::AppState,
    error::ErrService,
    features::book::{
        dto::{BookDto, CreateBookDto},
        service::BookService,
    },
};

use super::dto::{DeleteBookByIdDto, UpdateBookDto};

pub type SharedBookService<T> = Arc<Mutex<BookService<T>>>;

pub async fn create_booking(
    State(state): State<AppState>,
    Json(payload): Json<CreateBookDto>,
) -> Result<impl IntoResponse, ErrService> {
    let t0 = Instant::now();
    let service = state.book_service;
    let elapsed_lock = t0.elapsed();
    info!("Lock BookService duration {:?}", elapsed_lock);

    let t1 = Instant::now();
    let dto = service
        .book_room(&payload.room_name, &payload.user_name, &payload.date)
        .await?;

    let book_dto = BookDto {
        id: dto.id,
        room_name: dto.room_name.name,
        user_name: dto.user_name.name,
        date: dto.date.date,
    };
    let elapsed_inside = t1.elapsed();
    info!("BookService dto + book dto duration: {:?}", elapsed_inside);
    Ok(Json(book_dto))
}

pub async fn update_book(
    State(state): State<AppState>,
    Json(payload): Json<UpdateBookDto>,
) -> Result<impl IntoResponse, ErrService> {
    let service = state.book_service;

    let dto = service
        .update_book_by_id(
            payload.old_id,
            &payload.room_name,
            &payload.user_name,
            &payload.date,
        )
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
    let service = state.book_service;
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
    let service = state.book_service;

    service.delete_book_by_id(payload.id).await?;

    Ok(())
}

pub async fn delete_all_books(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ErrService> {
    let service = state.book_service;

    service.delete_all_book().await?;

    Ok(())
}
