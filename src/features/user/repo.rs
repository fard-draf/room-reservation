use crate::{
    domain::User,
    error::{ErrRepo, ErrService, ErrUser},
    features::user::dto::UserRowDto,
    infra::db::DBClient,
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepo {
    async fn insert_user(&self, user: &User) -> Result<User, ErrService>;
    async fn delete_user_by_name(&self, name: &str) -> Result<bool, ErrService>;
    async fn get_all_users(&self) -> Result<Vec<User>, ErrService>;
}

#[async_trait]
impl UserRepo for DBClient {
    async fn insert_user(&self, user: &User) -> Result<User, ErrService> {
        let row = sqlx::query_as::<_, UserRowDto>(
            "INSERT INTO users (user_name) VALUES ($1) RETURNING id, user_name",
        )
        .bind(&user.user_name.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| ErrRepo::Unreachable)?;

        let mut existing_users = vec![];
        for element in self.get_all_users().await? {
            existing_users.push(element.user_name);
        }
        if existing_users.contains(&user.user_name) {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }
        let user: User = row.try_into()?;
        Ok(user)
    }

    async fn delete_user_by_name(&self, name: &str) -> Result<bool, ErrService> {
        let result = sqlx::query("DELETE FROM users WHERE user_name = $1")
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrRepo::DoesntExist)?;

        if result.rows_affected() == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    async fn get_all_users(&self) -> Result<Vec<User>, ErrService> {
        let rows = sqlx::query_as::<_, UserRowDto>("SELECT id, user_name FROM users ")
            .fetch_all(&self.pool)
            .await
            .map_err(|_e| ErrRepo::Unreachable)?;

        let users = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(|_| ErrRepo::DoesntExist))
            .collect::<Result<_, _>>()?;

        Ok(users)
    }
}
