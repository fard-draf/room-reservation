#[cfg(test)]

mod test {

    use super::*;
    use crate::{
        domain::{User, UserID},
        error::{ErrService, ErrUser},
        features::user::{repo::UserRepo, service::UserService},
        infra::in_memory::in_memo_repo::InMemoryRepo,
    };
    use async_trait::async_trait;
    use uuid::Uuid;

    #[async_trait]
    impl UserRepo for InMemoryRepo<User> {
        async fn insert_user(&self, user: &User) -> Result<User, ErrService> {
            let vec = self.get_all_users().await?;
            if vec.iter().any(|x| x.user_name == user.user_name) {
                return Err(ErrService::User(ErrUser::AlreadyExist));
            }
            self.repo
                .lock()
                .unwrap()
                .insert(user.user_id.id, user.clone());
            Ok(user.clone())
        }
        async fn delete_user_by_name(&self, user: &str) -> Result<bool, ErrService> {
            let user = User::new(user)?;
            self.repo.lock().unwrap().remove(&user.user_id.id);
            Ok(true)
        }
        async fn update_user(&self, old_name: &str, new_name: &str) -> Result<User, ErrService> {
            let old_name = User::new(old_name)?;
            let new_name = User::new(new_name)?;
            let mut repo = self.repo.lock().unwrap();
            match repo.get_mut(&old_name.user_id.id) {
                Some(user) => {
                    user.user_name = new_name.user_name.clone();
                    Ok(new_name)
                }
                None => Err(ErrService::User(ErrUser::UserNotFound)),
            }
        }
        async fn get_all_users(&self) -> Result<Vec<User>, ErrService> {
            let vec = self.repo.lock().unwrap().values().cloned().collect();
            Ok(vec)
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ErrService> {
            let repo = self.repo.lock().unwrap();
            Ok(repo.get(&id).cloned())
        }
    }

    #[tokio::test]
    async fn print_all_users() {
        let repo = InMemoryRepo::new().await;
        let service = UserService::new(repo);

        let user_ok1 = service.add_user("Sophie").await;
        let user_ok2 = service.add_user("Jordan").await;

        println!("{:#?}", service)
    }

    #[tokio::test]
    async fn add_and_list_user() {
        let repo = InMemoryRepo::new().await;
        let service = UserService::new(repo);

        let user_ok1 = service.add_user("Sophie").await;
        let user_ok2 = service.add_user("Jordan").await;

        let user_err1 = service.add_user(" SOPHIE ").await; //already exist
        let user_err2 = service.add_user("SoPhiE").await; //already exist
        let user_err3 = service.add_user("A").await; //too short
        let user_err4 = service //too long
            .add_user("ABCDEFG       HIJKLMNO    PQRSTUVWXYZ")
            .await;

        assert!(user_ok1.is_ok());
        assert!(user_ok2.is_ok());

        assert!(user_err1.is_err());
        assert!(user_err2.is_err());
        assert!(user_err3.is_err());
        assert!(user_err4.is_err());

        assert!(service.is_exist_user("Jordan").await.is_ok());
        // assert!(!service.is_exist_user("     JORDAN   ").await.is_ok());

        // assert!(service.is_exist_user("Daniel").await.is_ok());
    }

    #[tokio::test]
    async fn delete_user_by_name() {
        let repo = InMemoryRepo::new().await;
        let mut service = UserService::new(repo);

        let user_ok1 = service.add_user("Sophie").await;
        let user_ok2 = service.add_user("Jordan").await;

        assert!(service.delete_user("Sophie").await.is_ok())
    }
}
