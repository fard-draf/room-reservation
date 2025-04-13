#[derive(Debug)]

pub enum ErrUser {
    InvalidNameTooShort,
    InvalidNameTooLong,
    InvalidID,
}
#[derive(Debug)]

pub enum ErrRoom {
    InvalidNameTooShort,
    InvalidNameTooLong,
    InvalidID,
}
#[derive(Debug)]

pub enum ErrBook {
    RoomNotFound,
    UserNotFound,
    AlreadyBooked,
    InvalidDateFormat,
    InvalidDate,
}
#[derive(Debug)]

pub enum ErrDB {
    Unreachable,
    DoesntExist,
}
#[derive(Debug)]
pub enum ErrDomain {
    BookCreation(ErrBook),
    RoomCreation(ErrRoom),
    UserCreation(ErrUser),
}
#[derive(Debug)]

pub enum ErrReservation {
    BookCreation(ErrBook),
    RoomCreation(ErrRoom),
    UserCreation(ErrUser),
}

impl From<ErrRoom> for ErrReservation {
    fn from(err: ErrRoom) -> Self {
        ErrReservation::RoomCreation(err)
    }
}

impl From<ErrUser> for ErrReservation {
    fn from(err: ErrUser) -> Self {
        ErrReservation::UserCreation(err)
    }
}
#[derive(Debug)]

pub enum ErrService {
    UserCreation(ErrUser),
    BookCreation(ErrBook),
    RoomCreation(ErrRoom),
    DbRequest(ErrDB),
    Domain(ErrDomain),
}

impl From<ErrUser> for ErrService {
    fn from(err: ErrUser) -> Self {
        ErrService::UserCreation(err)
    }
}

impl From<ErrBook> for ErrService {
    fn from(err: ErrBook) -> Self {
        ErrService::BookCreation(err)
    }
}

impl From<ErrRoom> for ErrService {
    fn from(err: ErrRoom) -> Self {
        ErrService::RoomCreation(err)
    }
}

impl From<ErrDB> for ErrService {
    fn from(err: ErrDB) -> Self {
        ErrService::DbRequest(err)
    }
}

impl From<ErrDomain> for ErrService {
    fn from(err: ErrDomain) -> Self {
        ErrService::Domain(err)
    }
}
