use crate::{domain::Book, error::ErrDB, features::book::dto::BookRowDto, infra::db::DBClient};

use async_trait::async_trait;

#[async_trait]
pub trait BookRepo {
    async fn insert_book(&self, book: &Book) -> Result<Book, ErrDB>;
    async fn delete_book_by_id(&self, id: i32) -> Result<bool, ErrDB>;
    async fn get_all_books(&self) -> Result<Vec<Book>, ErrDB>;
}

#[async_trait]
impl BookRepo for DBClient {
    async fn insert_book(&self, book: &Book) -> Result<Book, ErrDB> {
        let dto = sqlx::query_as::<_, BookRowDto>(
            "INSERT INTO books (room, user, date) VALUES ($1, $2, $3) RETURNING id, room, user, date"
            )
            .bind(&book.room_name.name)
            .bind(&book.user_name.name)
            .bind(&book.date.date)
            .fetch_one(&self.pool)
            .await
            .map_err(|_e| ErrDB::Unreachable)?;

        let book: Book = dto.try_into()?;

        Ok(book)
    }

    async fn delete_book_by_id(&self, id: i32) -> Result<bool, ErrDB> {
        let result = sqlx::query("DELETE FROM books WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrDB::DoesntExist)?;

        if result.rows_affected() == 0 {
            Err(ErrDB::DoesntExist)
        } else {
            Ok(true)
        }
    }

    async fn get_all_books(&self) -> Result<Vec<Book>, ErrDB> {
        let rows = sqlx::query_as::<_, BookRowDto>("SELECT id, room, user, date FROM books")
            .fetch_all(&self.pool)
            .await
            .map_err(|_e| ErrDB::Unreachable)?;

        let books = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(|_| ErrDB::DoesntExist))
            .collect::<Result<_, _>>()?;

        Ok(books)
    }
}
