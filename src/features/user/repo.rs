use crate::{
    domain::{User, UserID, UserName},
    error::{ErrRepo, ErrService, ErrUser},
    features::user::dto::UserRowDto,
    infra::db::DBClient,
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn insert_user(&self, user: &User) -> Result<User, ErrService>;
    async fn update_user(&self, id: Uuid, new_name: UserName) -> Result<User, ErrService>;
    async fn delete_user_by_name(&self, name: UserName) -> Result<bool, ErrService>;
    async fn get_all_users(&self) -> Result<Vec<User>, ErrService>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ErrService>;
}

#[async_trait]
impl UserRepo for DBClient {
    async fn insert_user(&self, user: &User) -> Result<User, ErrService> {
        let row = sqlx::query_as::<_, UserRowDto>(
            "INSERT INTO users (user_id, user_name) VALUES ($1, $2) RETURNING user_id, user_name",
        )
        .bind(user.user_id.id)
        .bind(&user.user_name.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| ErrRepo::Unreachable)?;

        let user: User = row.try_into()?;
        Ok(user)
    }

    async fn update_user(&self, id: Uuid, new_name: UserName) -> Result<User, ErrService> {
        let row = sqlx::query_as::<_, UserRowDto>(
            "UPDATE users SET user_name = $1 WHERE user_id = $2 RETURNING user_id, user_name",
        )
        .bind(new_name.name)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| ErrRepo::BadRequest)?;

        let user: User = User {
            user_id: UserID { id },
            user_name: UserName::new(&row.user_name)?,
        };
        Ok(user)
    }

    async fn delete_user_by_name(&self, name: UserName) -> Result<bool, ErrService> {
        let row = sqlx::query("DELETE FROM users WHERE user_name = $1")
            .bind(name.name)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrRepo::DoesntExist)?;

        Ok(row.rows_affected() > 0)
    }

    async fn get_all_users(&self) -> Result<Vec<User>, ErrService> {
        let rows = sqlx::query_as::<_, UserRowDto>("SELECT user_id, user_name FROM users ")
            .fetch_all(&self.pool)
            .await
            .map_err(|_e| ErrRepo::BadRequest)?;

        let users = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(|_| ErrRepo::DoesntExist))
            .collect::<Result<_, _>>()?;

        Ok(users)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ErrService> {
        let row = sqlx::query_as!(
            UserRowDto,
            "SELECT user_id, user_name FROM users WHERE user_id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_e| ErrRepo::DoesntExist)?;

        match row {
            Some(dto) => Ok(Some(User::try_from(dto)?)),
            None => Ok(None),
        }
    }
}
