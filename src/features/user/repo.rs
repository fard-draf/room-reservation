use crate::{
    domain::{User, UserName},
    error::{ErrRepo, ErrService, ErrUser},
    features::user::dto::UserRowDto,
    infra::db::DBClient,
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn insert_user(&self, user: &User) -> Result<User, ErrService>;
    async fn update_user(&self, old_name: &str, new_name: &str) -> Result<User, ErrService>;
    async fn delete_user_by_name(&self, name: &str) -> Result<bool, ErrService>;
    async fn get_all_users(&self) -> Result<Vec<User>, ErrService>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ErrService>;
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

    async fn update_user(&self, old_name: &str, new_name: &str) -> Result<User, ErrService> {
        let old_name = UserName::new(old_name)?;
        let new_name = UserName::new(new_name)?;

        let users = self.get_all_users().await?;

        let existing_user = users
            .iter()
            .find(|u| u.user_name == old_name)
            .cloned()
            .ok_or(ErrService::User(ErrUser::UserNotFound))?;

        if users.iter().any(|u| u.user_name == new_name) {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }

        let row = sqlx::query_as::<_, UserRowDto>(
            "UPDATE users SET user_name = $1 WHERE user_name = $2 RETURNING user_id, user_name",
        )
        .bind(new_name.name)
        .bind(old_name.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|_e| ErrRepo::BadRequest)?;

        // let user: User = row.try_into()?;
        let user: User = User {
            user_id: existing_user.user_id,
            user_name: UserName::new(&row.user_name)?,
        };
        Ok(user)
    }

    async fn delete_user_by_name(&self, name: &str) -> Result<bool, ErrService> {
        let result = sqlx::query("DELETE FROM users WHERE user_name = $1")
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(|_e| ErrRepo::DoesntExist)?;

        Ok(result.rows_affected() > 0)
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
