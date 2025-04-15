use chrono::{Local, NaiveDate};
use uuid::Uuid;

use crate::error::{ErrBook, ErrDomain, ErrRoom, ErrUser};

////////////////////////////USERS

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub id: i32,
    pub user_name: UserName,
}

impl User {
    // pub fn new(name: &str) -> Result<Self, ErrDomain> {
    //     Ok(Self {
    //         id: UserID::new(),
    //         name: UserName::new(name)?,
    //     })
    // }

    pub fn new(name: &str) -> Result<Self, ErrDomain> {
        Ok(Self {
            id: 0,
            user_name: UserName::new(name)?,
        })
    }
}

// #[derive(Debug, PartialEq, Clone)]
// struct UserID {
//     id: Uuid,
// }
// impl UserID {
//     fn new() -> Self {
//         Self { id: Uuid::new_v4() }
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub struct UserName {
    pub name: String,
}
impl UserName {
    pub fn new(name: &str) -> Result<Self, ErrDomain> {
        let cleaned_name = name.trim();
        if cleaned_name.len() <= 2 {
            return Err(ErrDomain::UserCreation(ErrUser::InvalidNameTooShort));
        } else if cleaned_name.len() >= 35 {
            return Err(ErrDomain::UserCreation(ErrUser::InvalidNameTooLong));
        } else {
            Ok(Self {
                name: cleaned_name.to_string(),
            })
        }
    }
}

////////////////////////////ROOMS

#[derive(Debug, PartialEq, Clone)]
pub struct Room {
    pub id: i32,
    pub room_name: RoomName,
}

impl Room {
    //     pub fn new(name: &str) -> Result<Self, ErrDomain> {
    //         Ok(Self {
    //             id: RoomID::new(),
    //             room_name: RoomName::new(name.to_string())?,
    //         })
    //     }
    // }

    pub fn new(name: &str) -> Result<Self, ErrDomain> {
        Ok(Self {
            id: 0,
            room_name: RoomName::new(name.to_string())?,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RoomName {
    pub name: String,
}

impl RoomName {
    pub fn new(mut name: String) -> Result<Self, ErrDomain> {
        name = name.trim().to_string();
        if name.len() <= 2 {
            return Err(ErrDomain::RoomCreation(ErrRoom::InvalidNameTooShort));
        } else if name.len() >= 17 {
            return Err(ErrDomain::RoomCreation(ErrRoom::InvalidNameTooLong));
        } else {
            Ok(Self { name })
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RoomID {
    id: Uuid,
}

impl RoomID {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

////////////////////////////REGISTERY BOOK

#[derive(Debug, sqlx::FromRow, PartialEq, Clone)]
pub struct Book {
    pub id: i32,
    pub room_name: Room,
    pub user_name: User,
    pub date: BookDate,
}
#[derive(Debug, PartialEq, Clone)]
pub struct BookDate {
    pub date: NaiveDate,
}

impl BookDate {
    pub fn new(input_date: &str) -> Result<Self, ErrDomain> {
        let actual_date = Local::now().date_naive();

        let cleaned: String = input_date.trim().replace("/", ".");

        if cleaned.len() != 8 {
            return Err(ErrDomain::BookCreation(ErrBook::InvalidDateFormat));
        }
        let reservation_date: NaiveDate = match NaiveDate::parse_from_str(&cleaned, "%d.%m.%y") {
            Ok(date) => date,
            Err(_e) => Err(ErrDomain::BookCreation(ErrBook::InvalidDateFormat))?,
        };

        if reservation_date < actual_date {
            return Err(ErrDomain::BookCreation(ErrBook::InvalidDate));
        }

        Ok(Self {
            date: reservation_date,
        })
    }
}
