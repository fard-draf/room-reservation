use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{Book, BookDate, RoomName, UserName},
    error::ErrService,
};

#[derive(Deserialize)]
pub struct CreateBookDto {
    pub room_name: String,
    pub user_name: String,
    pub date: String,
}

#[derive(Deserialize)]
pub struct DeleteBookByIdDto {
    pub id: i32,
}

#[derive(Serialize)]
pub struct BookDto {
    pub id: i32,
    pub room_name: String,
    pub user_name: String,
    pub date: NaiveDate,
}

// #[derive(Debug, sqlx::FromRow)]
#[derive(Debug, sqlx::FromRow)]

pub struct BookRowDto {
    pub id: i32,
    pub room_name: String,
    pub user_name: String,
    pub date: NaiveDate,
}

impl TryFrom<CreateBookDto> for Book {
    type Error = ErrService;

    fn try_from(dto: CreateBookDto) -> Result<Self, Self::Error> {
        Ok(Book {
            id: 0,
            room_name: RoomName::new(&dto.room_name)?,
            user_name: UserName::new(&dto.user_name)?,
            date: BookDate::new(&dto.date)?,
        })
    }
}

impl TryFrom<BookRowDto> for Book {
    type Error = ErrService;

    fn try_from(dto: BookRowDto) -> Result<Self, Self::Error> {
        Ok(Book {
            id: dto.id,
            room_name: RoomName::new(&dto.room_name)?,
            user_name: UserName::new(&dto.user_name)?,
            date: BookDate::new_from_naive(dto.date.clone())?,
        })
    }
}

impl From<Book> for BookDto {
    fn from(book: Book) -> Self {
        BookDto {
            id: book.id,
            room_name: book.room_name.name,
            user_name: book.user_name.name,
            date: book.date.date,
        }
    }
}
