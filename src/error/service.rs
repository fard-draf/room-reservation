use crate::error::{domain::*, http::*};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum ErrService {
    Book(ErrBook),
    User(ErrUser),
    Room(ErrRoom),
    Repo(ErrRepo),
    Domain(ErrDomain),
    Type(ErrType),
    IO(std::io::Error),
    Sqlx(sqlx::Error),
}

impl From<ErrUser> for ErrService {
    fn from(err: ErrUser) -> Self {
        ErrService::User(err)
    }
}

impl From<ErrBook> for ErrService {
    fn from(err: ErrBook) -> Self {
        ErrService::Book(err)
    }
}

impl From<ErrRoom> for ErrService {
    fn from(err: ErrRoom) -> Self {
        ErrService::Room(err)
    }
}

impl From<ErrRepo> for ErrService {
    fn from(err: ErrRepo) -> Self {
        ErrService::Repo(err)
    }
}

impl From<ErrDomain> for ErrService {
    fn from(err: ErrDomain) -> Self {
        ErrService::Domain(err)
    }
}

impl From<ErrType> for ErrService {
    fn from(err: ErrType) -> Self {
        ErrService::Type(err)
    }
}

impl From<sqlx::Error> for ErrService {
    fn from(err: sqlx::Error) -> Self {
        ErrService::Sqlx(err)
    }
}

impl From<std::io::Error> for ErrService {
    fn from(err: std::io::Error) -> Self {
        ErrService::IO(err)
    }
}

impl IntoResponse for ErrService {
    fn into_response(self) -> Response {
        match self {
            //  BOOK ERROR
            ErrService::Book(ErrBook::InvalidDateFormat) => bad_request("Invalid date format"),
            ErrService::Book(ErrBook::AlreadyBooked) => {
                conflict("Room already booked at this date")
            }
            ErrService::Book(ErrBook::InvalidDate) => unprocessable_entity("Date already past"),
            ErrService::Book(ErrBook::RoomNotFound) => not_found("Room not found in the system"),
            ErrService::Book(ErrBook::UserNotFound) => not_found("User not found in the system"),
            ErrService::Book(ErrBook::UnableToRead) => internal_error("Unable to read book"),
            ErrService::Book(ErrBook::InvalidID) => bad_request("Invalid ID, please check book ID"),
            //  USER ERROR
            ErrService::User(ErrUser::InvalidNameTooShort) => {
                unprocessable_entity("User's name is too short")
            }
            ErrService::User(ErrUser::InvalidNameTooLong) => {
                unprocessable_entity("User's name is too long")
            }
            ErrService::User(ErrUser::InvalidID) => not_found("User's ID not found in the system"),
            ErrService::User(ErrUser::UserNotFound) => not_found("User not found in the system"),
            ErrService::User(ErrUser::AlreadyExist) => conflict("User already exists"),

            //  ROOM ERROR
            ErrService::Room(ErrRoom::InvalidNameTooShort) => {
                unprocessable_entity("Room's name is too short")
            }
            ErrService::Room(ErrRoom::InvalidNameTooLong) => {
                unprocessable_entity("Room's name is too long")
            }
            ErrService::Room(ErrRoom::InvalidID) => bad_request("Invalid room's ID"),
            ErrService::Room(ErrRoom::AlreadyExist) => conflict("Room already exists"),
            ErrService::Room(ErrRoom::RoomNotFound) => not_found("Room not found in the system"),
            // TYPE ERROR
            ErrService::Type(ErrType::RawConversionFailed) => {
                internal_error("Raw conversion failed")
            }
            // DBREQUEST ERR
            ErrService::Repo(ErrRepo::BadRequest) => bad_request("Invalid request"),
            ErrService::Repo(ErrRepo::Unreachable) => {
                let body = Json(json!({"error" : "Database is unreachable"}));
                (StatusCode::SERVICE_UNAVAILABLE, body).into_response()
            }
            ErrService::Repo(ErrRepo::DoesntExist) => not_found("Not found in the system"),
            ErrService::Repo(ErrRepo::IsEmpty) => not_found("Already empty"),
            ErrService::Repo(ErrRepo::UnableToDelete) => unavailable("Unable to do this action"),
            // CATCH ALL
            _ => {
                let body = json!({ "error": "Service error" });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}
