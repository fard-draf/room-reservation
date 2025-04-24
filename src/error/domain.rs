#[derive(Debug)]
pub enum ErrUser {
    InvalidNameTooShort,
    InvalidNameTooLong,
    InvalidID,
    AlreadyExist,
    UserNotFound,
}

#[derive(Debug)]
pub enum ErrRoom {
    InvalidNameTooShort,
    InvalidNameTooLong,
    InvalidID,
    AlreadyExist,
    RoomNotFound,
}

#[derive(Debug)]
pub enum ErrBook {
    RoomNotFound,
    UserNotFound,
    AlreadyBooked,
    InvalidDateFormat,
    InvalidDate,
    InvalidID,
    UnableToRead,
    BookNotFound,
}

#[derive(Debug)]
pub enum ErrType {
    RawConversionFailed,
}

#[derive(Debug)]
pub enum ErrRepo {
    Unreachable,
    DoesntExist,
    RequestError,
    BadRequest,
    UnableToDelete,
    IsEmpty,
}

#[derive(Debug)]
pub enum ErrDomain {
    Book(ErrBook),
    Room(ErrRoom),
    User(ErrUser),
}
