#[cfg(test)]
mod test {

    use crate::{
        domain::{User, UserName},
        error::{ErrService, ErrUser},
        features::user::{repo::UserRepo, service::UserService},
        infra::in_memory::in_memo_repo::InMemoryRepo,
    };
    use async_trait::async_trait;
    use uuid::Uuid;

    #[async_trait]
    impl UserRepo for InMemoryRepo<User> {
        async fn insert_user(&self, user: &User) -> Result<User, ErrService> {
            self.repo.write().await.insert(user.clone());
            Ok(user.clone())
        }
        async fn delete_user_by_name(&self, user_name: UserName) -> Result<bool, ErrService> {
            let user = {
                let read_guard = self.repo.read().await;
                let user = read_guard
                    .iter()
                    .find(|v| v.user_name == user_name)
                    .cloned();
                user
            };

            let mut write_guard = self.repo.write().await;
            if let Some(key) = user {
                write_guard.remove(&key);
            } else {
                return Err(ErrService::User(ErrUser::UserNotFound));
            }

            Ok(true)
        }

        async fn update_user(&self, id: Uuid, new_name: UserName) -> Result<User, ErrService> {
            let old_user = {
                let read_guard = self.repo.read().await;
                let user = read_guard.iter().find(|u| u.user_id.id == id).cloned();
                user
            };

            let mut write_guard = self.repo.write().await;
            if let Some(user) = old_user {
                let new_user = User {
                    user_id: user.user_id.clone(),
                    user_name: new_name,
                };
                write_guard.remove(&user);
                write_guard.insert(new_user.clone());

                Ok(new_user)
            } else {
                Err(ErrService::User(ErrUser::UserNotFound))
            }
        }
        async fn get_all_users(&self) -> Result<Vec<User>, ErrService> {
            Ok(self.repo.read().await.iter().cloned().collect())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ErrService> {
            Ok(self
                .repo
                .read()
                .await
                .iter()
                .find(|u| u.user_id.id == id)
                .cloned())
        }

        async fn get_one_user(&self, user_name: &UserName) -> Result<User, ErrService> {
            if let Some(user) = self
                .repo
                .read()
                .await
                .iter()
                .find(|u| u.user_name == *user_name)
            {
                Ok(user.clone())
            } else {
                Err(ErrService::User(ErrUser::UserNotFound))
            }
        }
    }

    #[tokio::test]
    async fn print_all_users() {
        let service: UserService<InMemoryRepo<User>> = InMemoryRepo::init_user_service().await;

        assert!(service.add_user("Sophie").await.is_ok());
        assert!(service.add_user("Jordan").await.is_ok());
    }

    #[tokio::test]
    async fn add_and_list_user() {
        let service: UserService<InMemoryRepo<User>> = InMemoryRepo::init_user_service().await;

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

        assert!(
            service
                .is_exist_user(&UserName {
                    name: "Jordan".to_string()
                })
                .await
                .is_ok()
        );
        assert!(
            service
                .is_exist_user(&UserName {
                    name: "   JoRdAn    ".to_string()
                })
                .await
                .is_ok()
        );
        assert!(
            !service
                .is_exist_user(&UserName {
                    name: "Daniel".to_string()
                })
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn delete_user_by_name() {
        let service = InMemoryRepo::init_user_service().await;

        assert!(service.add_user("Sophie").await.is_ok());
        assert!(service.add_user("Jordan").await.is_ok());

        assert!(service.delete_user_by_name("Sophie").await.is_ok());
    }

    #[tokio::test]
    async fn update_user_name() {
        let repo = InMemoryRepo::new().await;
        let service = UserService::new(repo);

        assert!(service.add_user("Sophie").await.is_ok());
        assert!(
            service
                .is_exist_user(&UserName {
                    name: "Sophie".to_string()
                })
                .await
                .is_ok()
        );
        assert!(service.update_user("Sophie", "Alice").await.is_ok());
        assert!(
            service
                .is_exist_user(&UserName {
                    name: "Alice".to_string()
                })
                .await
                .is_ok()
        );
        assert!(service.update_user("ALICE", "CALISSE").await.is_ok());
        assert!(
            service
                .is_exist_user(&UserName {
                    name: "CALISSE".to_string()
                })
                .await
                .is_ok()
        );

        assert!(service.update_user("Unexisting", "Bob").await.is_err());
    }
}
