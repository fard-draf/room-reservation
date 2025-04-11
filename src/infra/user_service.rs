use std::error::Error;

use crate::{
    domain::User,
    repository::DBRepository,
};

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

    pub fn remove_user(&mut self, user: &User) -> Result<(), Box<dyn Error>> {
        self.repo.remove_data(&user)
    }
    pub fn list_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        self.repo.list()
    }
    pub fn is_exist_user(&self, user: &User) -> Result<bool, Box<dyn Error>> {
        self.repo.is_empty(user)
    }
}



#[cfg(test)]
mod test {} 

    #[test]
    fn add_and_list_user() -> Result<(), Box<dyn Error>>{
        let repo_user: InMemoryRepo<User> = InMemoryRepo::new();
        let mut user_service: UserService<InMemoryRepo<User>> = UserService::new(repo_user);

        let francois = user_service.add_user("Francois Fouesn")?;
        let jeanne = user_service.add_user("Jeanne Delcros")?;
        let david = user_service.add_user("David Durand")?;

        let users = user_service.list_users()?;

        assert_eq!(users.len(), 3);
        assert_eq!(users[0], francois);
        assert_eq!(users[1], jeanne);
        assert_eq!(users[2], david);

        Ok(())
    }