use crate::{
    domain::{User, UserName},
    error::{ErrRepo, ErrService, ErrUser},
    features::user::dto::UserRowDto,
    infra::db::DBClient,
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepo {
    async fn insert_user(&self, user: &User) -> Result<User, ErrService>;
    async fn update_user(&self, old_name: &str, new_name: &str) -> Result<User, ErrService>;
    async fn delete_user_by_name(&self, name: &str) -> Result<bool, ErrService>;
    async fn get_all_users(&self) -> Result<Vec<User>, ErrService>;
}

#[async_trait]
impl UserRepo for DBClient {
    async fn insert_user(&self, user: &User) -> Result<User, ErrService> {
        let mut existing_users: Vec<UserName> = Vec::new();
        for element in self.get_all_users().await? {
            existing_users.push(element.user_name);
        }
        if existing_users.contains(&user.user_name) {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }
        let row = sqlx::query_as::<_, UserRowDto>(
            "INSERT INTO users (user_name) VALUES ($1) RETURNING id, user_name",
        )
        .bind(&user.user_name.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| ErrRepo::Unreachable)?;

        let user: User = row.try_into()?;
        Ok(user)
    }

    async fn update_user(&self, old_name: &str, new_name: &str) -> Result<User, ErrService> {
        let old_name = UserName::new(old_name)?;
        let new_name = UserName::new(new_name)?;

        let users = self.get_all_users().await?;

        users
            .iter()
            .find(|u| u.user_name == old_name)
            .ok_or(ErrService::User(ErrUser::UserNotFound))?;

        if users.iter().any(|u| u.user_name == new_name) {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }

        let row = sqlx::query_as::<_, UserRowDto>(
            "UPDATE users SET user_name = $1 WHERE user_name = $2 RETURNING id, user_name",
        )
        .bind(new_name.name)
        .bind(old_name.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| ErrRepo::BadRequest)?;

        let user: User = row.try_into()?;
        Ok(user)
    }

    async fn delete_user_by_name(&self, name: &str) -> Result<bool, ErrService> {
        let result = sqlx::query("DELETE FROM users WHERE user_name = $1")
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrRepo::DoesntExist)?;

        Ok(!result.rows_affected() == 0)
    }

    async fn get_all_users(&self) -> Result<Vec<User>, ErrService> {
        let rows = sqlx::query_as::<_, UserRowDto>("SELECT id, user_name FROM users ")
            .fetch_all(&self.pool)
            .await
            .map_err(|_e| ErrRepo::BadRequest)?;

        let users = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(|_| ErrRepo::DoesntExist))
            .collect::<Result<_, _>>()?;

        Ok(users)
    }
}
