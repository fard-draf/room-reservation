use std::collections::HashSet;

use crate::{
    domain::{Book, BookDate, RoomName, UserName},
    error::{ErrBook, ErrDomain, ErrRepo, ErrService},
    features::{room::repo::RoomRepo, user::repo::UserRepo},
};

use super::repo::BookRepo;

use chrono::Local;

pub struct BookService<T> {
    repo: T,
}

impl<T> BookService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T> BookService<T>
where
    T: RoomRepo + UserRepo + BookRepo,
{
    pub async fn book_room(
        &self,
        room: &str,
        user: &str,
        desired_date: &str,
    ) -> Result<Book, ErrService> {
        let date =
            BookDate::new(desired_date).map_err(|_| ErrDomain::Book(ErrBook::InvalidDateFormat))?;

        if date.date < Local::now().date_naive() {
            return Err(ErrService::Book(ErrBook::InvalidDate));
        }

        let exist_room = self.repo.get_one_room(&RoomName::new(room)?).await.is_ok();
        if !exist_room {
            return Err(ErrService::Book(ErrBook::RoomNotFound));
        }

        let exist_user = self.repo.get_one_user(&UserName::new(user)?).await.is_ok();
        if !exist_user {
            return Err(ErrService::Book(ErrBook::UserNotFound));
        }

        let already_booked = self.repo.is_room_already_booked(room, &date.date).await?;
        if already_booked {
            return Err(ErrService::Book(ErrBook::AlreadyBooked));
        }

        let book = Book::new(room, user, date)?;
        let inserted_book = self.repo.insert_book(&book).await?;

        Ok(inserted_book)
    }

    pub async fn update_book_by_id(
        &self,
        old_book_id: i32,
        room: &str,
        user: &str,
        date: &str,
    ) -> Result<Book, ErrService> {
        let room = RoomName::new(room)?;
        let user = UserName::new(user)?;
        let date = BookDate::new(date)?;

        let book = Book {
            id: old_book_id,
            room_name: room,
            user_name: user,
            date,
        };

        let existing_id: HashSet<i32> = self
            .repo
            .get_all_books()
            .await?
            .into_iter()
            .map(|b| b.id)
            .collect();

        let existing_rooms: HashSet<RoomName> = self
            .repo
            .get_all_rooms()
            .await?
            .into_iter()
            .map(|r| r.room_name)
            .collect();

        let existing_users: HashSet<UserName> = self
            .repo
            .get_all_users()
            .await?
            .into_iter()
            .map(|u| u.user_name)
            .collect();

        if !existing_id.contains(&old_book_id) {
            return Err(ErrService::Book(ErrBook::UnableToRead));
        }

        if self
            .repo
            .get_all_books()
            .await?
            .iter()
            .any(|x| (x.date.date == book.date.date) && (x.room_name.name == book.room_name.name))
        {
            println!("Already booked");
            return Err(ErrService::Book(ErrBook::AlreadyBooked));
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

        let book = self.repo.update_book(&book).await?;
        Ok(book)
    }

    pub async fn list_book(&self) -> Result<Vec<Book>, ErrService> {
        self.repo.get_all_books().await
    }

    pub async fn delete_book_by_id(&self, book_id: i32) -> Result<(), ErrService> {
        let deleted = self.repo.delete_book_by_id(book_id).await?;
        if deleted {
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::UnableToDelete))
        }
    }

    pub async fn delete_all_book(&self) -> Result<(), ErrService> {
        let deleted = self.repo.delete_all_book().await?;
        if deleted {
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::UnableToDelete))
        }
    }
}
