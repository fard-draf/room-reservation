use std::error::Error;

use crate::{domain::User, repository::DBRepository};

use super::in_memo_repo::InMemoryRepo;

pub struct UserService<T> {
    repo: T,
}

impl<T> UserService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T: DBRepository<User>> UserService<T> {
    pub fn add_user(&mut self, user: &str) -> Result<User, Box<dyn Error>> {
        let user = User::new(user)?;
        self.repo.insert_data(&user)?;
        Ok(user)
    }

    pub fn remove_user(&mut self, user: &str) -> Result<(), Box<dyn Error>> {
        let user_list = self.repo.list()?;
        if let Some(data) = user_list.iter().find(|x| x.name.name == user) {
            return Ok(self.repo.remove_data(data)?);
        } else {
            Err("User doesn't exist".into())
        }
    }
    pub fn list_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        self.repo.list()
    }
    pub fn is_exist_user(&self, user: &str) -> Result<bool, Box<dyn Error>> {
        let user_list = self.repo.list()?;
        if user_list.iter().any(|x| x.name.name == user) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
