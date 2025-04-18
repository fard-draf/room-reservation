use crate::{
    domain::{Book, RoomName, UserName},
    error::{ErrBook, ErrRepo, ErrService, ErrType},
    features::{book::dto::BookRowDto, room::repo::RoomRepo, user::repo::UserRepo},
    infra::db::DBClient,
};

use async_trait::async_trait;
use chrono::Local;

#[async_trait]
pub trait BookRepo {
    async fn insert_book(&self, book: &Book) -> Result<Book, ErrService>;
    async fn get_all_books(&self) -> Result<Vec<Book>, ErrService>;
    async fn delete_book_by_id(&self, id: i32) -> Result<bool, ErrService>;
    async fn delete_all_book(&self) -> Result<bool, ErrService>;
}

#[async_trait]
impl BookRepo for DBClient {
    async fn insert_book(&self, book: &Book) -> Result<Book, ErrService> {
        let mut existing_rooms: Vec<RoomName> = Vec::new();
        for element in self.get_all_rooms().await? {
            existing_rooms.push(element.room_name);
        }

        let mut existing_users: Vec<UserName> = vec![];
        for element in self.get_all_users().await? {
            existing_users.push(element.user_name);
        }

        if book.date.date < Local::now().date_naive() {
            return Err(ErrService::Book(ErrBook::InvalidDate));
        }
        if !existing_rooms.contains(&book.room_name) {
            return Err(ErrService::Book(ErrBook::RoomNotFound));
        }
        if !existing_users.contains(&book.user_name) {
            return Err(ErrService::Book(ErrBook::UserNotFound));
        }
        let row = sqlx::query_as::<_, BookRowDto>(
                "INSERT INTO books (room_name, user_name, date) VALUES ($1, $2, $3) RETURNING id, room_name, user_name, date"
                )
                .bind(&book.room_name.name)
                .bind(&book.user_name.name)
                .bind(book.date.date)
                .fetch_one(&self.pool)
                .await
                .map_err(|e: sqlx::Error| {
                        eprintln!("Insert error: {:?}", e);
                        ErrService::Repo(ErrRepo::BadRequest)
                     })?;

        let book: Book = match row.try_into() {
            Ok(book) => book,
            Err(_) => return Err(ErrService::Type(ErrType::RawConversionFailed)),
        };

        Ok(book)
    }

    async fn get_all_books(&self) -> Result<Vec<Book>, ErrService> {
        let rows =
            sqlx::query_as::<_, BookRowDto>("SELECT id, room_name, user_name, date FROM books")
                .fetch_all(&self.pool)
                .await
                .map_err(|_e| ErrType::RawConversionFailed)?;

        let books = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(|_| ErrBook::UnableToRead))
            .collect::<Result<_, _>>()?;

        Ok(books)
    }

    async fn delete_book_by_id(&self, id: i32) -> Result<bool, ErrService> {
        let result = sqlx::query("DELETE FROM books WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrRepo::BadRequest)?;

        if result.rows_affected() == 0 {
            Err(ErrService::Repo(ErrRepo::DoesntExist))
        } else {
            Ok(true)
        }
    }

    async fn delete_all_book(&self) -> Result<bool, ErrService> {
        let result = sqlx::query("DELETE FROM books")
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrRepo::BadRequest)?;

        if result.rows_affected() == 0 {
            Err(ErrService::Repo(ErrRepo::DoesntExist))
        } else {
            Ok(true)
        }
    }
}
