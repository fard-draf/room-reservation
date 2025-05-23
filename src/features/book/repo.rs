use crate::{
    domain::{Book, BookDate, RoomName, UserName},
    error::{ErrBook, ErrRepo, ErrService, ErrType},
    features::book::dto::BookRowDto,
    infra::db::DBClient,
};

use async_trait::async_trait;
use chrono::NaiveDate;

#[async_trait]
pub trait BookRepo {
    async fn insert_book(&self, book: &Book) -> Result<Book, ErrService>;
    async fn update_book(&self, book: &Book) -> Result<Book, ErrService>;
    async fn get_all_books(&self) -> Result<Vec<Book>, ErrService>;
    async fn delete_book_by_id(&self, id: i32) -> Result<bool, ErrService>;
    async fn delete_all_book(&self) -> Result<bool, ErrService>;
    async fn get_one_book(&self, book: &Book) -> Result<Option<Book>, ErrService>;
    async fn is_room_already_booked(
        &self,
        room: &str,
        date: &NaiveDate,
    ) -> Result<bool, ErrService>;
}

#[async_trait]
impl BookRepo for DBClient {
    async fn insert_book(&self, book: &Book) -> Result<Book, ErrService> {
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

    async fn update_book(&self, book: &Book) -> Result<Book, ErrService> {
        let row = sqlx::query_as::<_, BookRowDto>(
            "UPDATE books SET room_name = $2, user_name = $3, date = $4 WHERE id = $1  RETURNING id, room_name, user_name, date",
            )
            .bind(book.id)
            .bind(&book.room_name.name)
            .bind(&book.user_name.name)
            .bind(book.date.date)
            .fetch_one(&self.pool)
            .await
            .map_err(|_e| ErrRepo::BadRequest)?;

        // let book: Book = row.try_into()?;
        let book: Book = Book {
            id: book.id,
            room_name: RoomName {
                name: row.room_name,
            },
            user_name: UserName {
                name: row.user_name,
            },
            date: BookDate { date: row.date },
        };
        Ok(book)
    }

    async fn get_all_books(&self) -> Result<Vec<Book>, ErrService> {
        let rows = sqlx::query_as::<_, BookRowDto>(
            "SELECT id, room_name, user_name, date FROM books ORDER BY date",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_e| ErrType::RawConversionFailed)?;

        let books: Vec<Book> = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(|_| ErrBook::UnableToRead))
            .collect::<Result<_, _>>()?;

        Ok(books)
    }

    async fn get_one_book(&self, book: &Book) -> Result<Option<Book>, ErrService> {
        let row = sqlx::query_as::<_, BookRowDto>("SELECT * FROM books WHERE id = $1")
            .bind(book.id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| ErrRepo::BadRequest)?;

        if let Some(row_book) = row {
            let book = Book {
                id: row_book.id,
                room_name: RoomName {
                    name: (row_book.room_name),
                },
                user_name: UserName {
                    name: (row_book.user_name),
                },
                date: BookDate {
                    date: (row_book.date),
                },
            };
            Ok(Some(book))
        } else {
            Ok(None)
        }
    }

    async fn delete_book_by_id(&self, id: i32) -> Result<bool, ErrService> {
        let result = sqlx::query("DELETE FROM books WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrRepo::BadRequest)?;

        if result.rows_affected() == 0 {
            Err(ErrService::Book(ErrBook::InvalidID))
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
            Err(ErrService::Repo(ErrRepo::IsEmpty))
        } else {
            Ok(true)
        }
    }

    async fn is_room_already_booked(
        &self,
        room: &str,
        date: &NaiveDate,
    ) -> Result<bool, ErrService> {
        let result = sqlx::query("SELECT 1 FROM books WHERE room_name = $1 AND date = $2 LIMIT 1")
            .bind(room)
            .bind(date)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| ErrService::Repo(ErrRepo::BadRequest))?;

        Ok(result.is_some())
    }
}
