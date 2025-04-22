use std::collections::HashSet;

use crate::{
    domain::{User, UserID, UserName},
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
        let existing_users: HashSet<UserName> = self
            .repo
            .get_all_users()
            .await?
            .into_iter()
            .map(|u| u.user_name)
            .collect();

        if existing_users.contains(&user.user_name) {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }
        let user = self.repo.insert_user(&user).await?;
        Ok(user)
    }

    pub async fn update_user(&self, old_name: &str, new_name: &str) -> Result<User, ErrService> {
        let old_name = UserName::new(&old_name.trim().to_lowercase())?;
        let new_name = UserName::new(&new_name.trim().to_lowercase())?;

        // let users = self.repo.get_all_users().await?;

        // let existing_user = users
        //     .iter()
        //     .find(|u| u.user_name == old_name)
        //     .cloned()
        //     .ok_or(ErrService::User(ErrUser::UserNotFound))?;

        let existing_users: HashSet<(UserName, UserID)> = self
            .repo
            .get_all_users()
            .await?
            .into_iter()
            .map(|u| (u.user_name, u.user_id))
            .collect();

        let existing_user = existing_users
            .iter()
            .find(|u| u.0 == old_name)
            .cloned()
            .ok_or(ErrService::User(ErrUser::UserNotFound))?;

        if existing_users.iter().any(|u| u.0 == new_name) {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }

        let user = self.repo.update_user(existing_user.1.id, new_name).await?;
        Ok(user)
    }

    pub async fn delete_user(&mut self, user_name: &str) -> Result<(), ErrService> {
        let user_name = UserName::new(user_name)?;

        let deleted = self.repo.delete_user_by_name(user_name).await?;
        if deleted {
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::DoesntExist))
        }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, ErrService> {
        self.repo.get_all_users().await
    }

    pub async fn is_exist_user(&self, user: &str) -> Result<bool, ErrService> {
        let user = UserName::new(user)?;
        let user_list = self.repo.get_all_users().await?;

        Ok(user_list.iter().any(|x| x.user_name == user))
    }
}
