use chrono::NaiveDate;
use uuid::Uuid;

use crate::error::{ErrBook, ErrDomain, ErrRoom, ErrUser};

////////////////////////////USERS

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct User {
    pub user_id: UserID,
    pub user_name: UserName,
}

impl User {
    pub fn new(name: &str) -> Result<Self, ErrDomain> {
        Ok(Self {
            user_id: UserID::new(),
            user_name: UserName::new(name)?,
        })
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct UserID {
    pub id: Uuid,
}
impl UserID {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

impl Default for UserID {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, sqlx::FromRow)]
pub struct UserName {
    pub name: String,
}
impl UserName {
    pub fn new(name: &str) -> Result<Self, ErrDomain> {
        let cleaned_name = name.trim().to_lowercase();
        if cleaned_name.len() <= 2 {
            Err(ErrDomain::User(ErrUser::InvalidNameTooShort))
        } else if cleaned_name.len() >= 35 {
            return Err(ErrDomain::User(ErrUser::InvalidNameTooLong));
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
            room_name: RoomName::new(name)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, sqlx::FromRow)]
pub struct RoomName {
    pub name: String,
}

impl RoomName {
    pub fn new(mut name: &str) -> Result<Self, ErrDomain> {
        name = name.trim();
        if name.len() <= 2 {
            Err(ErrDomain::Room(ErrRoom::InvalidNameTooShort))
        } else if name.len() >= 17 {
            return Err(ErrDomain::Room(ErrRoom::InvalidNameTooLong));
        } else {
            Ok(Self {
                name: name.to_string().to_uppercase(),
            })
        }
    }
}

// #[derive(Debug, PartialEq, Clone)]
// // pub struct RoomID {
// //     id: Uuid,
// // }

// // impl RoomID {
// //     pub fn new() -> Self {
// //         Self { id: Uuid::new_v4() }
// //     }
// // }

////////////////////////////REGISTERY BOOK
#[derive(Debug, sqlx::FromRow, Eq, Hash, PartialEq, Clone)]
pub struct Book {
    pub id: i32,
    pub room_name: RoomName,
    pub user_name: UserName,
    pub date: BookDate,
}

impl Book {
    pub fn new(room_name: &str, user_name: &str, date: BookDate) -> Result<Self, ErrDomain> {
        Ok(Self {
            id: 0,
            room_name: RoomName::new(room_name)?,
            user_name: UserName::new(user_name)?,
            date,
        })
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, sqlx::FromRow)]
pub struct BookDate {
    pub date: NaiveDate,
}

impl BookDate {
    pub fn new(input_date: &str) -> Result<Self, ErrDomain> {
        let cleaned: String = input_date.trim().replace("/", ".");

        if cleaned.len() != 8 {
            return Err(ErrDomain::Book(ErrBook::InvalidDateFormat));
        }
        let reservation_date: NaiveDate = match NaiveDate::parse_from_str(&cleaned, "%d.%m.%y") {
            Ok(date) => date,
            Err(_e) => Err(ErrDomain::Book(ErrBook::InvalidDateFormat))?,
        };

        Ok(Self {
            date: reservation_date,
        })
    }

    pub fn from_naive(input_date: NaiveDate) -> Result<Self, ErrDomain> {
        Ok(Self { date: input_date })
    }
}
