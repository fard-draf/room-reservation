use serde::{Deserialize, Serialize};

use crate::{
    domain::{User, UserName},
    error::ErrDomain,
};

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub user_name: String,
}

pub struct DeleteUserByIdDto {
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct UserDto {
    pub id: i32,
    pub user_name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserRowDto {
    pub id: i32,
    pub user_name: String,
}

impl TryFrom<CreateUserDto> for User {
    type Error = ErrDomain;

    fn try_from(dto: CreateUserDto) -> Result<Self, Self::Error> {
        Ok(User {
            id: 0,
            user_name: UserName::new(&dto.user_name)?,
        })
    }
}

impl TryFrom<UserRowDto> for User {
    type Error = ErrDomain;

    fn try_from(dto: UserRowDto) -> Result<Self, Self::Error> {
        Ok(User {
            id: dto.id,
            user_name: UserName::new(&dto.user_name)?,
        })
    }
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id,
            user_name: user.user_name.name,
        }
    }
}
