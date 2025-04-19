#[cfg(test)]

mod test {

    use super::*;
    use crate::{
        domain::{User, UserName},
        error::{ErrService, ErrUser},
        features::user::{repo::UserRepo, service::UserService},
        infra::in_memory::in_memo_repo::InMemoryRepo,
    };
    use async_trait::async_trait;

    #[async_trait]
    impl UserRepo for InMemoryRepo<User> {
        async fn insert_user(&self, user: &User) -> Result<User, ErrService> {
            let insert_user = self.repo.lock().unwrap().insert(user.id, user.clone());
            match insert_user {
                Some(user) => Ok(user),
                None => Err(ErrService::User(ErrUser::InvalidID)),
            }
        }
        async fn delete_user_by_name(&self, user: &str) -> Result<bool, ErrService> {
            let user = User::new(user)?;
            self.repo.lock().unwrap().remove(&user.id);
            Ok(true)
        }
        async fn update_user(&self, old_name: &str, new_name: &str) -> Result<User, ErrService> {
            let old_name = User::new(old_name)?;
            let new_name = User::new(new_name)?;
            let mut repo = self.repo.lock().unwrap();
            match repo.get_mut(&old_name.id) {
                Some(user) => {
                    user.user_name = new_name.user_name.clone();
                    Ok(new_name)
                }
                None => Err(ErrService::User(ErrUser::UserNotFound)),
            }
        }
        async fn get_all_users(&self) -> Result<Vec<User>, ErrService> {
            let mut vec = Vec::<User>::new();
            for (k, v) in self.repo.lock().unwrap().iter() {
                vec.push(v.clone());
            }
            Ok(vec)
        }
    }
}
