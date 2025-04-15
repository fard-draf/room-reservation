use crate::{
    domain::User,
    error::{ErrDB, ErrService},
};

use super::user_repo::UserRepo;

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
        let user = User::new(name)?;
        let user = self.repo.insert_user(&user).await?;
        Ok(user)
    }

    pub async fn delete_user(&mut self, name: &str) -> Result<(), ErrService> {
        let deleted = self.repo.delete_user_by_id(name).await?;
        if deleted {
            Ok(())
        } else {
            Err(ErrService::DbRequest(ErrDB::DoesntExist))
        }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, ErrDB> {
        self.repo.get_all_users().await
    }

    pub async fn is_exist_user(&self, user: &str) -> Result<bool, ErrService> {
        let user_list = self.repo.get_all_users().await?;
        if user_list.iter().any(|x| x.user_name.name == user) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
