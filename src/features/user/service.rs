use super::repo::UserRepo;
use crate::{
    domain::{User, UserID, UserName},
    error::{ErrRepo, ErrService, ErrUser},
};
use dashmap::DashSet;
use tracing::info;

#[derive(Debug)]
pub struct UserService<T> {
    repo: T,
    cache: DashSet<User>,
}

impl<T> UserService<T> {
    pub fn new(repo: T) -> Self {
        Self {
            repo,
            cache: DashSet::new(),
        }
    }
}

impl<T: UserRepo> UserService<T> {
    pub async fn add_user(&self, name: &str) -> Result<User, ErrService> {
        let user: User = User::new(name)?;

        if self.is_exist_user(&user.user_name).await? {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }

        let user = self.repo.insert_user(&user).await?;
        self.cache.insert(user.clone());
        info!("cache lenght: {}", self.cache.len());
        Ok(user)
    }

    pub async fn update_user(&self, old_name: &str, new_name: &str) -> Result<User, ErrService> {
        let old_name = UserName::new(&old_name.trim().to_lowercase())?;
        let new_name = UserName::new(&new_name.trim().to_lowercase())?;

        if !self.is_exist_user(&old_name).await? {
            return Err(ErrService::User(ErrUser::UserNotFound));
        }

        if self.is_exist_user(&new_name).await? {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }

        let maybe_user = {
            self.cache
                .iter()
                .find(|u| u.user_name == old_name)
                .map(|u| u.key().clone())
        };

        if let Some(old_user) = maybe_user {
            let updated_user = self.repo.update_user(old_user.user_id.id, new_name).await?;

            self.cache.remove(&old_user);
            self.cache.insert(updated_user.clone());

            Ok(updated_user)
        } else {
            Err(ErrService::User(ErrUser::UserNotFound))
        }
    }

    pub async fn delete_user_by_name(&self, user_name: &str) -> Result<(), ErrService> {
        let user_name = UserName::new(user_name)?;
        let deleted = self.repo.delete_user_by_name(user_name.clone()).await?;

        if deleted {
            self.cache.retain(|u| u.user_name != user_name);
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::DoesntExist))
        }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, ErrService> {
        self.repo.get_all_users().await
    }

    pub async fn is_exist_user(&self, user_name: &UserName) -> Result<bool, ErrService> {
        Ok(self.cache.iter().any(|u| u.user_name == *user_name))
    }

    pub async fn get_user_by_user_struct_on_cache(&self, user: &User) -> Result<User, ErrService> {
        if self.cache.contains(user) {
            Err(ErrService::User(ErrUser::AlreadyExist))
        } else if let Some(value) = self.cache.get(&user.clone()) {
            Ok(value.clone())
        } else {
            return Err(ErrService::User(ErrUser::UserNotFound));
        }
    }

    pub async fn get_user_by_id_on_cache(
        &self,
        user_id: UserID,
    ) -> Result<Option<User>, ErrService> {
        if let Some(user_by_id) = self.cache.iter().find(|x| x.user_id == user_id) {
            Ok(Some(user_by_id.clone()))
        } else {
            Ok(None)
        }
    }

    pub async fn populate_cache(&self) -> Result<(), ErrService> {
        let users = self.repo.get_all_users().await?;
        for element in users {
            let user = User {
                user_id: element.user_id,
                user_name: element.user_name,
            };
            self.cache.insert(user);
        }
        println!("UserService cache lenght: {}", self.cache.len());
        Ok(())
    }
}
