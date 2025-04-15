use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{Book, BookDate, Room, User},
    error::ErrDomain,
};

#[derive(Deserialize)]
pub struct CreateBookDto {
    pub room_name: String,
    pub user_name: String,
    pub date: String,
}

#[derive(Serialize)]
pub struct BookDto {
    pub id: i32,
    pub room_name: String,
    pub user_name: String,
    pub date: String,
}

#[derive(Debug, sqlx::FromRow)]

pub struct BookRowDto {
    pub id: i32,
    pub room_name: String,
    pub user_name: String,
    pub date: NaiveDate,
}

impl TryFrom<CreateBookDto> for Book {
    type Error = ErrDomain;

    fn try_from(dto: CreateBookDto) -> Result<Self, Self::Error> {
        Ok(Book {
            id: 0,
            room_name: Room::new(&dto.room_name)?,
            user_name: User::new(&dto.user_name)?,
            date: BookDate::new(&dto.date)?,
        })
    }
}

impl TryFrom<BookRowDto> for Book {
    type Error = ErrDomain;

    fn try_from(dto: BookRowDto) -> Result<Self, Self::Error> {
        Ok(Book {
            id: dto.id,
            room_name: Room::new(&dto.room_name)?,
            user_name: User::new(&dto.user_name)?,
            date: BookDate::new(&dto.date.to_string())?,
        })
    }
}

impl From<Book> for BookDto {
    fn from(book: Book) -> Self {
        BookDto {
            id: book.id,
            room_name: book.room_name.room_name.name,
            user_name: book.user_name.user_name.name,
            date: book.date.date.format("%d.%m.%y").to_string(),
        }
    }
}
