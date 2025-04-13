use crate::{
    domain::{Book, BookDate, Room, User},
    error::{ErrBook, ErrDomain, ErrService},
    repository::DBRepository,
};

pub struct RegService<T> {
    repo: T,
}

impl<T> RegService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T: DBRepository<Book>> RegService<T> {
    pub fn book_room(
        &mut self,
        room: &Room,
        user: &User,
        desired_date: &str,
    ) -> Result<(), ErrService> {
        let date = BookDate::new(desired_date)
            .map_err(|_| ErrDomain::BookCreation(ErrBook::InvalidDateFormat))?;
        let book = Book {
            room: room.clone(),
            user: user.clone(),
            date,
        };

        let all_book = self.repo.list()?;
        let is_already_booked = all_book
            .iter()
            .any(|x| (x.date.date == book.date.date) && (x.room.name.name == room.name.name));

        if is_already_booked {
            println!("Already booked");
            return Ok(());
        }

        self.repo.insert_data(&book)?;
        println!("{:?} reserved on {:?}", room.name, desired_date);

        Ok(())
    }

    pub fn print_book(&self) -> Result<Vec<Book>, ErrService> {
        Ok(self.repo.list()?)
    }
}
