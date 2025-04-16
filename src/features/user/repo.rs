use crate::{domain::User, error::ErrDB, features::user::dto::UserRowDto, infra::db::DBClient};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepo {
    async fn insert_user(&self, user: &User) -> Result<User, ErrDB>;
    async fn delete_user_by_name(&self, name: &str) -> Result<bool, ErrDB>;
    async fn get_all_users(&self) -> Result<Vec<User>, ErrDB>;
}

#[async_trait]
impl UserRepo for DBClient {
    async fn insert_user(&self, user: &User) -> Result<User, ErrDB> {
        let row = sqlx::query_as::<_, UserRowDto>(
            "INSERT INTO users (user_name) VALUES ($1) RETURNING id, user_name",
        )
        .bind(&user.user_name.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| ErrDB::Unreachable)?;

        let user: User = row.try_into()?;
        Ok(user)
    }

    async fn delete_user_by_name(&self, name: &str) -> Result<bool, ErrDB> {
        let result = sqlx::query("DELETE FROM users WHERE user_name = $1")
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrDB::DoesntExist)?;

        if result.rows_affected() == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    async fn get_all_users(&self) -> Result<Vec<User>, ErrDB> {
        let rows = sqlx::query_as::<_, UserRowDto>("SELECT id, user_name FROM users ")
            .fetch_all(&self.pool)
            .await
            .map_err(|_e| ErrDB::Unreachable)?;

        let users = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(|_| ErrDB::DoesntExist))
            .collect::<Result<_, _>>()?;

        Ok(users)
    }
}
