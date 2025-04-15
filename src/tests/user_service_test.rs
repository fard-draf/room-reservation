#[cfg(test)]
mod test {
    use crate::{
        domain::{User, UserName},
        error::ErrService,
        infra::{in_memo_repo::InMemoryRepo, user_service::UserService},
        tests::test_helpers::{default_user1, default_user2, init_user_service},
    };

    #[tokio::test]

    async fn add_and_list_user() -> Result<(), ErrService> {
        let mut user_service = init_user_service().await?;

        let user1_name = &(default_user1().await?).name.name;
        let user2_name = &(default_user2().await?).name.name;

        assert!(user_service.add_new_user(&user1_name).await.is_ok());
        assert!(user_service.add_new_user(&user2_name).await.is_ok());

        let users = user_service.list_users().await?;

        assert_eq!(users.len(), 2);

        Ok(())
    }

    #[tokio::test]

    async fn create_an_invalid_user_too_short_or_too_long() -> Result<(), ErrService> {
        assert!(UserName::new("\t \t \t \t Daniel \t \t \t ").is_ok());
        assert!(UserName::new("A").is_err());
        assert!(
            UserName::new("Jean-Edouard De la Portaillere de la Manchada de la Cabesa").is_err()
        );

        Ok(())
    }

    #[tokio::test]

    async fn remove_an_existing_and_unexisting_user() -> Result<(), ErrService> {
        let mut user_service: UserService<InMemoryRepo<User>> = init_user_service().await?;

        user_service.add_new_user("Boris").await?;

        assert!(user_service.is_exist_user("Boris").await.is_ok());
        assert_eq!(user_service.is_exist_user("Joris").await?, false);

        assert!(user_service.remove_user("Boris").await.is_ok());
        assert!(user_service.remove_user("Joris").await.is_err());

        Ok(())
    }
}
