use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

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
    InvalidID,
}
#[derive(Debug)]

pub enum ErrDB {
    Unreachable,
    DoesntExist,
    RequestError,
    BadRequest,
}

impl IntoResponse for ErrDB {
    fn into_response(self) -> Response {
        let body = Json(json!({ "error": "Database error" }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl From<ErrDomain> for ErrDB {
    fn from(_err: ErrDomain) -> Self {
        ErrDB::Unreachable
    }
}
#[derive(Debug)]
pub enum ErrDomain {
    Book(ErrBook),
    Room(ErrRoom),
    User(ErrUser),
}

impl From<ErrBook> for ErrDomain {
    fn from(err: ErrBook) -> Self {
        ErrDomain::Book(err)
    }
}
#[derive(Debug)]

pub enum ErrReservation {
    Book(ErrBook),
    Room(ErrRoom),
    User(ErrUser),
}

impl From<ErrRoom> for ErrReservation {
    fn from(err: ErrRoom) -> Self {
        ErrReservation::Room(err)
    }
}

impl From<ErrUser> for ErrReservation {
    fn from(err: ErrUser) -> Self {
        ErrReservation::User(err)
    }
}

impl From<ErrBook> for ErrReservation {
    fn from(err: ErrBook) -> Self {
        ErrReservation::Book(err)
    }
}
#[derive(Debug)]

pub enum ErrService {
    Book(ErrBook),
    User(ErrUser),
    Room(ErrRoom),
    DBRequest(ErrDB),
    Domain(ErrDomain),
}

fn bad_request(msg: &str) -> Response {
    (StatusCode::BAD_REQUEST, Json(json!({"error" : msg }))).into_response()
}

fn confict(msg: &str) -> Response {
    (StatusCode::CONFLICT, Json(json!({"error" : msg}))).into_response()
}

impl IntoResponse for ErrService {
    fn into_response(self) -> Response {
        match self {
            ///// BOOK ERROR
            ErrService::Book(ErrBook::InvalidDateFormat) => bad_request("Invalid date format"),
            ErrService::Book(ErrBook::AlreadyBooked) => confict("Room already booked at this date"),
            ErrService::Book(ErrBook::InvalidDate) => bad_request("Date already past"),
            ErrService::Book(ErrBook::RoomNotFound) => bad_request("Room not found in the system"),
            ErrService::Book(ErrBook::UserNotFound) => bad_request("User not found in the system"),
            ErrService::Book(ErrBook::InvalidID) => {
                bad_request("Invalid ID request, please check book ID")
            }

            ///// USER ERROR
            ErrService::User(ErrUser::InvalidNameTooShort) => {
                bad_request("User's name is too short")
            }
            ErrService::User(ErrUser::InvalidNameTooLong) => bad_request("User's name is too long"),
            ErrService::User(ErrUser::InvalidID) => {
                bad_request("User's ID not found in the system")
            }

            ///// ROOM ERROR
            ErrService::Room(ErrRoom::InvalidNameTooShort) => {
                bad_request("Room's name is too short")
            }
            ErrService::Room(ErrRoom::InvalidNameTooLong) => bad_request("Room's name is too long"),
            ErrService::Room(ErrRoom::InvalidID) => bad_request("Invalid room's ID"),

            ///// DBREQUEST ERR
            ErrService::DBRequest(ErrDB::BadRequest) => bad_request("Invalid request"),
            ErrService::DBRequest(ErrDB::Unreachable) => {
                let body = Json(json!({"error" : "Database is unreachable"}));
                (StatusCode::SERVICE_UNAVAILABLE, body).into_response()
            }
            ErrService::DBRequest(ErrDB::DoesntExist) => {
                let body = Json(json!({"error" : "Not found in the system"}));
                (StatusCode::SERVICE_UNAVAILABLE, body).into_response()
            }

            // ErrService::Domain(ErrDomain::Book(ErrBook::InvalidDate)) => bad_request("Date is already past"),
            _ => {
                let body = json!({ "error": "Service error" });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
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

impl From<ErrDB> for ErrService {
    fn from(err: ErrDB) -> Self {
        ErrService::DBRequest(err)
    }
}

impl From<ErrDomain> for ErrService {
    fn from(err: ErrDomain) -> Self {
        ErrService::Domain(err)
    }
}

/////////HTTP_ERROR

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ErrorResponse { error: message });

        (status, body).into_response()
    }
}
