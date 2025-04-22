use crate::{
    domain::{Book, BookDate, RoomName, UserName},
    error::{ErrBook, ErrDomain, ErrRepo, ErrService},
    features::{book::dto::BookRowDto, room::repo::RoomRepo, user::repo::UserRepo},
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

        let book = Book::new(room, user, date)?;

        let is_already_booked =
            self.repo.get_all_books().await?.iter().any(|x| {
                (x.date.date == book.date.date) && (x.room_name.name == *room.to_string())
            });

        if is_already_booked {
            println!("Already booked");
            return Err(ErrService::Book(ErrBook::AlreadyBooked));
        }

        let mut existing_rooms: Vec<RoomName> = Vec::new();
        for element in self.repo.get_all_rooms().await? {
            existing_rooms.push(element.room_name);
        }

        let mut existing_users: Vec<UserName> = vec![];
        for element in self.repo.get_all_users().await? {
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

        let book = self.repo.insert_book(&book).await?;
        println!("{:?} reserved on {:?}, id: {}", room, desired_date, book.id);

        Ok(book)
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
        let book = Book::new(&room.name, &user.name, date)?;
        let mut id_book = Vec::new();
        for element in self.repo.get_all_books().await? {
            id_book.push(element.id)
        }
        if !id_book.contains(&old_book_id) {
            return Err(ErrService::Book(ErrBook::UnableToRead));
        }

        let book = self.repo.update_book(&book).await?;
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
