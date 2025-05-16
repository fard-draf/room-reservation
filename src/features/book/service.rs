use dashmap::{self, DashSet};
use tracing::info;

use crate::{
    domain::{Book, BookDate, RoomName, UserName},
    error::{ErrBook, ErrDomain, ErrRepo, ErrService},
    features::{room::repo::RoomRepo, user::repo::UserRepo},
};

use super::repo::BookRepo;

use chrono::Local;

pub struct BookService<T> {
    repo: T,
    cache: DashSet<Book>,
}

impl<T> BookService<T> {
    pub fn new(repo: T) -> Self {
        Self {
            repo,
            cache: DashSet::new(),
        }
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
        let room_name = RoomName::new(room)?;
        let user_name = UserName::new(user)?;

        if date.date < Local::now().date_naive() {
            return Err(ErrService::Book(ErrBook::InvalidDate));
        }

        let exist_room = self.repo.get_one_room(&room_name).await.is_ok();
        if !exist_room {
            return Err(ErrService::Book(ErrBook::RoomNotFound));
        }

        let exist_user = self.repo.get_one_user(&user_name).await.is_ok();
        if !exist_user {
            return Err(ErrService::Book(ErrBook::UserNotFound));
        }

        if self.is_exist_book(&room_name, &date).await? {
            return Err(ErrService::Book(ErrBook::AlreadyBooked));
        }

        let book = Book::new(room, user, date)?;
        let inserted_book = self.repo.insert_book(&book).await?;
        self.cache.insert(book.clone());

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

        if !self.is_exist_book_id(&old_book_id).await? {
            return Err(ErrService::Book(ErrBook::InvalidID));
        }

        if self.is_exist_book(&room, &date).await? {
            return Err(ErrService::Book(ErrBook::AlreadyBooked));
        }

        if date.date < Local::now().date_naive() {
            return Err(ErrService::Book(ErrBook::InvalidDate));
        }

        let existing_users = self.repo.get_one_user(&user).await?;
        if existing_users.user_name != user {
            return Err(ErrService::Book(ErrBook::UserNotFound));
        }

        let existing_rooms = self.repo.get_one_room(&room).await?;
        if existing_rooms.room_name != room {
            return Err(ErrService::Book(ErrBook::RoomNotFound));
        }

        let book = Book {
            id: old_book_id,
            room_name: room,
            user_name: user,
            date,
        };

        let book = self.repo.update_book(&book).await?;
        self.cache.remove(&book);
        self.cache.insert(book.clone());
        Ok(book)
    }

    pub async fn list_book(&self) -> Result<Vec<Book>, ErrService> {
        self.repo.get_all_books().await
    }

    pub async fn list_book_by_cache(&self) -> Result<Vec<Book>, ErrService> {
        Ok(self
            .cache
            .iter()
            .map(|item| item.clone())
            .collect::<Vec<Book>>())
    }

    pub async fn delete_book_by_id(&self, book_id: i32) -> Result<(), ErrService> {
        let deleted = self.repo.delete_book_by_id(book_id).await?;
        if deleted {
            self.cache.retain(|x| x.id != book_id);
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::UnableToDelete))
        }
    }

    pub async fn is_exist_book(
        &self,
        room: &RoomName,
        date: &BookDate,
    ) -> Result<bool, ErrService> {
        Ok(self
            .cache
            .iter()
            .any(|b| (b.date == *date) && (b.room_name == *room)))
    }

    pub async fn is_exist_book_id(&self, id: &i32) -> Result<bool, ErrService> {
        Ok(self.cache.iter().any(|b| b.id == *id))
    }

    pub async fn delete_all_book(&self) -> Result<(), ErrService> {
        let deleted = self.repo.delete_all_book().await?;
        if deleted {
            self.cache.clear();
            info!("Cache lenght: {:?}", self.cache.len());
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::UnableToDelete))
        }
    }

    pub async fn populate_cache(&self) -> Result<(), ErrService> {
        self.list_book().await?.into_iter().for_each(|e| {
            let book = Book {
                id: e.id,
                room_name: e.room_name,
                user_name: e.user_name,
                date: e.date,
            };
            self.cache.insert(book);
        });

        info!("BookService cache lenght: {}", self.cache.len());
        Ok(())
    }
}
