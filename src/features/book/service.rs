use chrono::Local;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::repo::BookRepo;
use crate::{
    domain::{Book, BookDate, RoomName, UserName},
    error::{ErrBook, ErrDomain, ErrRepo, ErrService},
    features::{
        room::{repo::RoomRepo, service::RoomService},
        user::{repo::UserRepo, service::UserService},
    },
    infra::db::DBClient,
};

#[derive(Debug)]
pub struct BookService<T> {
    pub repo: T,
    pub user_service: Arc<UserService<DBClient>>,
    pub room_service: Arc<RoomService<DBClient>>,
    pub cache: RwLock<HashSet<Book>>,
}

impl<T> BookService<T> {
    pub fn new(
        repo: T,
        user_service: Arc<UserService<DBClient>>,
        room_service: Arc<RoomService<DBClient>>,
    ) -> Self {
        Self {
            repo,
            user_service,
            room_service,
            cache: RwLock::new(HashSet::new()),
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

        let book = Book::new(room, user, date)?;

        let is_already_booked =
            self.repo.get_all_books().await?.iter().any(|x| {
                (x.date.date == book.date.date) && (x.room_name.name == *room.to_string())
            });

        if is_already_booked {
            println!("Already booked");
            return Err(ErrService::Book(ErrBook::AlreadyBooked));
        }

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

        let book = Book {
            id: old_book_id,
            room_name: room,
            user_name: user,
            date,
        };

        if self.is_exist_book(&book).await? {
            return Err(ErrService::Book(ErrBook::AlreadyBooked));
        }

        let maybe_room = {
            let guard = self
                .room_service
                .get_room_from_cache(&book.room_name)
                .await
                .is_ok();
            guard
        };

        if !maybe_room {
            return Err(ErrService::Book(ErrBook::RoomNotFound));
        }

        let maybe_user = {
            let guard = self
                .user_service
                .get_user_from_cache(&book.user_name)
                .await
                .is_ok();
            guard
        };

        if !maybe_user {
            return Err(ErrService::Book(ErrBook::UserNotFound));
        };

        let maybe_id = {
            let guard = self.cache.read().await;
            guard.iter().any(|b| b.id == book.id)
        };

        if !maybe_id {
            return Err(ErrService::Book(ErrBook::InvalidID));
        };
        // let existing_id: HashSet<i32> = self
        //     .repo
        //     .get_all_books()
        //     .await?
        //     .into_iter()
        //     .map(|b| b.id)
        //     .collect();

        // let existing_rooms: HashSet<RoomName> = self
        //     .repo
        //     .get_all_rooms()
        //     .await?
        //     .into_iter()
        //     .map(|r| r.room_name)
        //     .collect();

        // let existing_users: HashSet<UserName> = self
        //     .repo
        //     .get_all_users()
        //     .await?
        //     .into_iter()
        //     .map(|u| u.user_name)
        //     .collect();

        // if !existing_id.contains(&old_book_id) {
        //     return Err(ErrService::Book(ErrBook::UnableToRead));
        // }

        // if self
        //     .repo
        //     .get_all_books()
        //     .await?
        //     .iter()
        //     .any(|x| (x.date.date == book.date.date) && (x.room_name.name == book.room_name.name))
        // {
        //     println!("Already booked");
        //     return Err(ErrService::Book(ErrBook::AlreadyBooked));
        // }

        if book.date.date < Local::now().date_naive() {
            return Err(ErrService::Book(ErrBook::InvalidDate));
        }

        // if !existing_rooms.contains(&book.room_name) {
        //     return Err(ErrService::Book(ErrBook::RoomNotFound));
        // }
        // if !existing_users.contains(&book.user_name) {
        //     return Err(ErrService::Book(ErrBook::UserNotFound));
        // }

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

    pub async fn is_exist_book(&self, data: &Book) -> Result<bool, ErrService> {
        let book_list = self.cache.read().await;
        if book_list
            .iter()
            .any(|x| (x.room_name == data.room_name) && (x.date == data.date))
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn get_cache_book_by_book_struct(&self, book: &Book) -> Result<Book, ErrService> {
        if let Some(value) = self.cache.read().await.get(&book.clone()) {
            Ok(value.clone())
        } else {
            Err(ErrService::Book(ErrBook::BookNotFound))
        }
    }

    pub async fn get_cache_book_by_id(&self, book_id: i32) -> Result<Option<Book>, ErrService> {
        if let Some(book_by_id) = self.cache.read().await.iter().find(|x| x.id == book_id) {
            Ok(Some(book_by_id.clone()))
        } else {
            Ok(None)
        }
    }

    pub async fn populate_cache(&mut self) -> Result<(), ErrService> {
        println!("[populate_cache] instance BookService: {:p}", self);
        let books = self.repo.get_all_books().await?;
        for element in books {
            let book = Book {
                id: element.id,
                room_name: element.room_name,
                user_name: element.user_name,
                date: element.date,
            };
            self.cache.write().await.insert(book);
        }
        Ok(())
    }
}
