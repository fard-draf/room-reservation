use serde::{Deserialize, Serialize};

use crate::{
    domain::{User, UserName},
    error::ErrDomain,
};

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub user_name: String,
}

#[derive(Deserialize)]
pub struct UpdateUserNameDto {
    pub old_name: String,
    pub new_name: String,
}

#[derive(Deserialize)]
pub struct DeleteUserByIdDto {
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct UserDto {
    pub id: i32,
    pub user_name: String,
}

#[derive(Serialize)]
pub struct UpdateUserDto {
    pub id: i32,
    pub new_name: String,
}
#[derive(Debug, sqlx::FromRow)]
pub struct UserUpdateRowDto {
    pub user_name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserRowDto {
    pub id: i32,
    pub user_name: String,
}

impl TryFrom<UpdateUserNameDto> for User {
    type Error = ErrDomain;

    fn try_from(dto: UpdateUserNameDto) -> Result<Self, Self::Error> {
        Ok(User {
            id: 0,
            user_name: UserName::new(&dto.new_name)?,
        })
    }
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

impl From<User> for UpdateUserDto {
    fn from(user: User) -> Self {
        UpdateUserDto {
            id: user.id,
            new_name: user.user_name.name,
        }
    }
}
