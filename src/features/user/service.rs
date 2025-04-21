use crate::{
    domain::{User, UserName},
    error::{ErrRepo, ErrService, ErrUser},
};

use super::repo::UserRepo;

#[derive(Debug)]
pub struct UserService<T> {
    repo: T,
}

impl<T> UserService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T: UserRepo> UserService<T> {
    pub async fn add_user(&self, name: &str) -> Result<User, ErrService> {
        let user = User::new(&name.trim().to_lowercase())?;
        let mut existing_users: Vec<UserName> = Vec::new();
        for element in self.repo.get_all_users().await? {
            existing_users.push(element.user_name);
        }
        if existing_users.contains(&user.user_name) {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }
        let user = self.repo.insert_user(&user).await?;
        Ok(user)
    }

    pub async fn update_user(&self, old_name: &str, new_name: &str) -> Result<User, ErrService> {
        let old_name = UserName::new(&old_name.trim().to_lowercase())?;
        let new_name = UserName::new(&new_name.trim().to_lowercase())?;

        let users = self.repo.get_all_users().await?;

        let existing_user = users
            .iter()
            .find(|u| u.user_name == old_name)
            .cloned()
            .ok_or(ErrService::User(ErrUser::UserNotFound))?;

        if users.iter().any(|u| u.user_name == new_name) {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }

        let user = self
            .repo
            .update_user(existing_user.user_id.id, new_name)
            .await?;
        Ok(user)
    }

    pub async fn delete_user(&mut self, name: &str) -> Result<(), ErrService> {
        let deleted = self.repo.delete_user_by_name(name).await?;
        if deleted {
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::DoesntExist))
        }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, ErrService> {
        self.repo.get_all_users().await
    }

    pub async fn is_exist_user(&self, user: &str) -> Result<(), ErrService> {
        let user_list = self.repo.get_all_users().await?;
        if user_list
            .iter()
            .any(|x| x.user_name.name == user.trim().to_lowercase())
        {
            Ok(())
        } else {
            Err(ErrService::User(crate::error::ErrUser::UserNotFound))
        }
    }
}
