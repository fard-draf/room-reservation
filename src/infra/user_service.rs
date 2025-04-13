use crate::{
    domain::User,
    error::{ErrDB, ErrDomain, ErrService, ErrUser},
    repository::DBRepository,
};

pub struct UserService<T> {
    repo: T,
}

impl<T> UserService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T: DBRepository<User>> UserService<T> {
    pub fn add_new_user(&mut self, user: &str) -> Result<User, ErrService> {
        let user = User::new(user)?;
        self.repo.insert_data(&user)?;
        Ok(user)
    }

    pub fn remove_user(&mut self, user: &str) -> Result<(), ErrService> {
        let user_list = self.repo.list()?;
        if let Some(data) = user_list.iter().find(|x| x.name.name == user) {
            return Ok(self.repo.remove_data(data)?);
        } else {
            Err(ErrService::DbRequest(ErrDB::Unreachable))
        }
    }

    pub fn list_users(&self) -> Result<Vec<User>, ErrDB> {
        self.repo.list()
    }

    pub fn is_exist_user(&self, user: &str) -> Result<bool, ErrService> {
        let user_list = self.repo.list()?;
        if user_list.iter().any(|x| x.name.name == user) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
