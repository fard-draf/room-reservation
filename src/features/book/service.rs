use crate::{
    domain::{Book, BookDate},
    error::{ErrBook, ErrDomain, ErrRepo, ErrService},
};

use super::repo::BookRepo;

pub struct BookService<T> {
    repo: T,
}

impl<T> BookService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T: BookRepo> BookService<T> {
    pub async fn book_room(
        &self,
        room: &str,
        user: &str,
        desired_date: &str,
    ) -> Result<Book, ErrService> {
        let date =
            BookDate::new(desired_date).map_err(|_| ErrDomain::Book(ErrBook::InvalidDateFormat))?;

        let book = Book::new(room, user, date)?;

        let is_already_booked = self
            .repo
            .get_all_books()
            .await?
            .iter()
            .any(|x| (x.date.date == book.date.date) && (x.room_name.name == room.to_string()));

        if is_already_booked {
            println!("Already booked");
            return Err(ErrService::Book(ErrBook::AlreadyBooked));
        }

        let book = self.repo.insert_book(&book).await?;
        println!("{:?} reserved on {:?}, id: {}", room, desired_date, book.id);

        Ok(book)
    }

    pub async fn list_book(&self) -> Result<Vec<Book>, ErrService> {
        self.repo.get_all_books().await
    }

    pub async fn delete_book_by_id(&mut self, book_id: i32) -> Result<(), ErrService> {
        let deleted = self.repo.delete_book_by_id(book_id).await?;
        if deleted {
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::UnableToDelete))
        }
    }

    pub async fn delete_all_book(&mut self) -> Result<(), ErrService> {
        let deleted = self.repo.delete_all_book().await?;
        if deleted {
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::UnableToDelete))
        }
    }

}
