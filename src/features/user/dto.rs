use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::{User, UserID, UserName},
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
    pub user_id: Uuid,
    pub user_name: String,
}

#[derive(Serialize)]
pub struct UpdateUserDto {
    pub user_id: Uuid,
    pub new_name: String,
}
#[derive(Debug, sqlx::FromRow)]
pub struct UserUpdateRowDto {
    pub user_name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserRowDto {
    pub user_id: Uuid,
    pub user_name: String,
}

// impl TryFrom<UpdateUserNameDto> for User {
//     type Error = ErrDomain;

//     fn try_from(dto: UpdateUserNameDto) -> Result<Self, Self::Error> {
//         Ok(User {
//             user_id: UserID::new(),
//             user_name: UserName::new(&dto.new_name)?,
//         })
//     }
// }

impl TryFrom<CreateUserDto> for User {
    type Error = ErrDomain;

    fn try_from(dto: CreateUserDto) -> Result<Self, Self::Error> {
        Ok(User {
            user_id: UserID::new(),
            user_name: UserName::new(&dto.user_name)?,
        })
    }
}

impl TryFrom<UserRowDto> for User {
    type Error = ErrDomain;

    fn try_from(dto: UserRowDto) -> Result<Self, Self::Error> {
        Ok(User {
            user_id: UserID { id: dto.user_id },
            user_name: UserName::new(&dto.user_name)?,
        })
    }
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            user_id: user.user_id.id,
            user_name: user.user_name.name,
        }
    }
}

impl From<User> for UpdateUserDto {
    fn from(user: User) -> Self {
        UpdateUserDto {
            user_id: user.user_id.id,
            new_name: user.user_name.name,
        }
    }
}
