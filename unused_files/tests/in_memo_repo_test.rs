#[cfg(test)]

mod test {
    use crate::{
        domain::User,
        error::ErrService,
        infra::in_memory::in_memo_repo::InMemoryRepo,
        tests::test_helpers::{default_users, init_inmemory_repo},
    };

    async fn initialize_repo<T>() -> Result<(InMemoryRepo<T>, User, User), ErrService> {
        let repo = InMemoryRepo::<User>::new().await;
        let repo = init_inmemory_repo().await?;
        let (user1, user2) = default_users().await?;
        Ok((repo, user1, user2))
    }

    #[tokio::test]

    async fn insert_data_in_memory_and_find_it() -> Result<(), ErrService> {
        let (mut repo, user1, user2) = initialize_repo().await?;

        assert!(repo.insert_data(&user1).await.is_ok());
        assert_eq!(repo.is_exist(&user1).await?, true);
        assert_eq!(repo.is_exist(&user2).await?, false);

        Ok(())
    }

    #[tokio::test]
    async fn list_data_and_remove_data() -> Result<(), ErrService> {
        let (mut repo, user1, user2) = initialize_repo().await?;

        assert!(repo.insert_data(&user1).await.is_ok());
        assert!(repo.list().await.is_ok());
        assert!(repo.remove_data(&user1).await.is_ok());
        assert!(repo.remove_data(&user2).await.is_err());

        Ok(())
    }
}
