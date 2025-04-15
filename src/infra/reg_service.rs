use crate::{
    domain::{Book, BookDate, Room, User},
    error::{ErrBook, ErrDomain, ErrService},
    // repository::DBRepository,
    infra::in_memo_repo::InMemoryRepo,
};

pub struct RegService {
    repo: InMemoryRepo<Book>,
}

impl RegService {
    pub fn new(repo: InMemoryRepo<Book>) -> Self {
        Self { repo }
    }
}

impl RegService {
    pub async fn book_room(
        &mut self,
        room: &Room,
        user: &User,
        desired_date: &str,
    ) -> Result<(), ErrService> {
        let date = BookDate::new(desired_date)
            .map_err(|_| ErrDomain::BookCreation(ErrBook::InvalidDateFormat))?;
        let book = Book {
            id: 1,
            room_name: room.clone(),
            user_name: user.clone(),
            date,
        };

        let all_book = self.repo.list().await?;
        let is_already_booked = all_book.iter().any(|x| {
            (x.date.date == book.date.date) && (x.room_name.room_name.name == room.room_name.name)
        });

        if is_already_booked {
            println!("Already booked");
            return Err(ErrService::BookCreation(ErrBook::AlreadyBooked));
        }

        self.repo.insert_data(&book).await?;
        println!("{:?} reserved on {:?}", room.room_name, desired_date);

        Ok(())
    }

    pub async fn print_book(&self) -> Result<Vec<Book>, ErrService> {
        Ok(self.repo.list().await?)
    }
}
