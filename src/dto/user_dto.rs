use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub name: String,
}

#[derive(Serialize)]
pub struct UserDto {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateRoomDto {
    pub name: String,
}

#[derive(Serialize)]
pub struct RoomDto {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateBookingDto {
    pub room_name: String,
    pub user_name: String,
    pub date: String,
}

#[derive(Serialize)]
pub struct BookingDto {
    pub room_name: String,
    pub user_name: String,
    pub date: String,
}
