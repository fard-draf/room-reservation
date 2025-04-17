use crate::{
    domain::Book,
    error::{ErrBook, ErrDB, ErrService},
    features::book::dto::BookRowDto,
    infra::db::DBClient,
};

use async_trait::async_trait;
use chrono::Local;

#[async_trait]
pub trait BookRepo {
    async fn insert_book(&self, book: &Book) -> Result<Book, ErrService>;
    async fn delete_book_by_id(&self, id: i32) -> Result<bool, ErrService>;
    async fn get_all_books(&self) -> Result<Vec<Book>, ErrService>;
}

#[async_trait]
impl BookRepo for DBClient {
    async fn insert_book(&self, book: &Book) -> Result<Book, ErrService> {
        if book.date.date > Local::now().date_naive() {   
            let row = sqlx::query_as::<_, BookRowDto>(
                "INSERT INTO books (room_name, user_name, date) VALUES ($1, $2, $3) RETURNING id, room_name, user_name, date"
                )
                .bind(&book.room_name.name)
                .bind(&book.user_name.name)
                .bind(&book.date.date)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| {
                        println!("Insert error: {:?}", e);
                        ErrService::DBRequest(ErrDB::BadRequest)
                     })?;
            
            let book: Book = row.try_into()?;
            
            return Ok(book)
            }  else {
                
             return Err(ErrService::Book(ErrBook::InvalidDate));
            }
        // if book.date.date < Local::now().date_naive() {
        //     return Err(ErrService::Book(ErrBook::InvalidDate));
        // }

        // Ok(book)
    }

    async fn delete_book_by_id(&self, id: i32) -> Result<bool, ErrService> {
        let result = sqlx::query("DELETE FROM books WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrDB::BadRequest)?;

        if result.rows_affected() == 0 {
            Err(ErrService::DBRequest(ErrDB::DoesntExist))
        } else {
            Ok(true)
        }
    }

    async fn get_all_books(&self) -> Result<Vec<Book>, ErrService> {
        let rows =
            sqlx::query_as::<_, BookRowDto>("SELECT id, room_name, user_name, date FROM books")
                .fetch_all(&self.pool)
                .await
                .map_err(|_e| ErrDB::Unreachable)?;

        let books = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(|_| ErrBook::InvalidID))
            .collect::<Result<_, _>>()?;

        Ok(books)
    }
}
