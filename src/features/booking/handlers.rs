use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};


use std:: sync::Arc;
use tokio::sync::Mutex;

use crate::{
    app::state::AppState, 
    features::booking::{
        dto::{
            CreateBookDto, 
            BookDto,
        },
        repo::BookRepo,
        service::BookingService,
    },
    domain::Book,
    error::{
        ErrDB, 
        ErrService
    }, 
    
};

pub type SharedRoomService<T> = Arc<Mutex<BookingService<T>>>;

pub async fn create_booking<T> (
    State(state): State<AppState>,
    Json(payload): Json<CreateBookDto>,
) -> Result<impl IntoResponse, ErrService>
where 
    T: BookRepo + Send + 'static
    {
        let mut service = state.book_service.lock().await;

        let dto: Book = service
            .book_room(&payload.room_name, &payload.user_name, &payload.date)
            .await?;
        let book_dto = BookDto {
            id: dto.id
        };

        Ok(Json(room_dto))
}

pub async fn list_room(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ErrDB> 

{
    let service = state.room_service.lock().await;
    let rooms = service.list_rooms().await?;

    let dto: Vec<RoomDto> = rooms
        .into_iter()
        .map(|u| RoomDto {
            id: u.id,
            room_name: u.room_name.name,
        })
        .collect();

    Ok(Json(dto))
}

pub async fn delete_room(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoomDto>,
) -> Result<impl IntoResponse, ErrService> 

{
    let mut service = state.room_service.lock().await;
    
    let dto = service
        .delete_room(&payload.room_name)
        .await?;


    Ok(())

}
