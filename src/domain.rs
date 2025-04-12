use std::error::Error;

use chrono::NaiveDate;
use uuid::Uuid;

////////////////////////////USERS/////////////////////////

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    id: UserID,
    pub name: UserName,
}

impl User {
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            id: UserID::new(),
            name: UserName::new(name.to_string())?,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct UserID {
    id: Uuid,
}
impl UserID {
    fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserName {
    pub name: String,
}
impl UserName {
    pub fn new(name: String) -> Result<Self, Box<dyn Error>> {
        if name.len() <= 2 {
            return Err("Invalid name".into());
        } else {
            Ok(Self { name })
        }
    }
}

///////////////////////ROOMS/////////////////////////
#[derive(Debug, PartialEq, Clone)]
pub struct Room {
    id: RoomID,
    pub name: RoomName,
}

impl Room {
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            id: RoomID::new(),
            name: RoomName::new(name.to_string())?,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RoomName {
    pub name: String,
}

impl RoomName {
    pub fn new(name: String) -> Result<Self, Box<dyn Error>> {
        if name.len() <= 2 {
            return Err("Invalid name".into());
        } else {
            Ok(Self { name })
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct RoomID {
    id: Uuid,
}

impl RoomID {
    fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

//////////////////REGISTERY BOOK//////////////////////

#[derive(Debug, PartialEq, Clone)]
pub struct Book {
    pub room: Room,
    pub user: User,
    pub date: BookDate,
}
#[derive(Debug, PartialEq, Clone)]
pub struct BookDate {
    pub date: NaiveDate,
}

impl BookDate {
    pub fn new(input_date: &str) -> Result<Self, Box<dyn Error>> {
        let reservation_date = NaiveDate::parse_from_str(input_date, "%d.%m.%y")?;
        Ok(Self {
            date: reservation_date,
        })
    }
}
