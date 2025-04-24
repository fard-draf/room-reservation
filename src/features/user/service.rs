use crate::{
    domain::{User, UserID, UserName},
    error::{ErrRepo, ErrService, ErrUser},
};

use std::collections::HashSet;
use tokio::sync::RwLock;

use super::repo::UserRepo;

#[derive(Debug)]
pub struct UserService<T> {
    pub repo: T,
    cache: RwLock<HashSet<User>>,
}

impl<T> UserService<T> {
    pub fn new(repo: T) -> Self {
        Self {
            repo,
            cache: RwLock::new(HashSet::new()),
        }
    }

    pub async fn get_user_from_cache(&self, name: &UserName) -> Result<Option<User>, ErrService> {
        let guard = self.cache.read().await;
        Ok(guard.iter().find(|u| u.user_name == *name).cloned())
    }
}

impl<T: UserRepo> UserService<T> {
    pub async fn add_user(&self, name: &str) -> Result<User, ErrService> {
        let user = User::new(name)?;

        let is_existing_user = self.is_exist_user(&user.user_name).await?;

        if is_existing_user {
            return Err(ErrService::User(ErrUser::AlreadyExist));
        }
        let user = self.repo.insert_user(&user).await?;
        let mut cache = self.cache.write().await;
        cache.insert(user.clone());
        tracing::info!(
            "User added to cache: {:?} cache has now {} entries",
            user,
            cache.len()
        );
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
            let guard = self.cache.read().await;
            guard.iter().find(|u| u.user_name == old_name).cloned()
        };

        if let Some(old_user) = maybe_user {
            let updated_user = self.repo.update_user(old_user.user_id.id, new_name).await?;

            let mut cache = self.cache.write().await;
            cache.remove(&old_user);
            cache.insert(updated_user.clone());

            Ok(updated_user)
        } else {
            Err(ErrService::User(ErrUser::UserNotFound))
        }
    }

    pub async fn delete_user_by_name(&self, user_name: &str) -> Result<(), ErrService> {
        let user_name = UserName::new(user_name)?;

        let deleted = self.repo.delete_user_by_name(user_name.clone()).await?;
        if deleted {
            let mut guard = self.cache.write().await;
            guard.retain(|u| u.user_name != user_name);
            Ok(())
        } else {
            Err(ErrService::Repo(ErrRepo::DoesntExist))
        }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, ErrService> {
        self.repo.get_all_users().await
    }

    pub async fn is_exist_user(&self, user: &UserName) -> Result<bool, ErrService> {
        let user_list = self.cache.read().await;
        if user_list.iter().any(|u| u.user_name == *user) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn get_cache_user_by_user_struct(&self, user: &User) -> Result<User, ErrService> {
        if self.cache.read().await.contains(user) {
            Err(ErrService::User(ErrUser::AlreadyExist))
        } else if let Some(value) = self.cache.read().await.get(&user.clone()) {
            Ok(value.clone())
        } else {
            return Err(ErrService::User(ErrUser::UserNotFound));
        }
    }

    pub async fn get_cache_user_by_id(&self, user_id: UserID) -> Result<Option<User>, ErrService> {
        if let Some(user_by_id) = self
            .cache
            .read()
            .await
            .iter()
            .find(|x| x.user_id == user_id)
        {
            Ok(Some(user_by_id.clone()))
        } else {
            Ok(None)
        }
    }

    pub async fn populate_cache(&self) -> Result<(), ErrService> {
        println!("[populate_cache] instance UserService: {:p}", self);
        let users = self.repo.get_all_users().await?;
        for element in users {
            let room = User {
                user_id: element.user_id,
                user_name: element.user_name,
            };
            self.cache.write().await.insert(room);
        }
        Ok(())
    }
}
