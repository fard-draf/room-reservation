#[cfg(test)]

mod test {
    use crate::{
        domain::*,
        error::ErrService,
        infra::in_memo_repo::InMemoryRepo,
        repository::*,
        tests::test_helpers::{default_users, init_inmemory_repo, init_user_service},
    };

    fn initialize_repo<T>() -> Result<(InMemoryRepo<T>, User, User), ErrService> {
        let repo = init_inmemory_repo()?;
        let (user1, user2) = default_users()?;
        Ok((repo, user1, user2))
    }

    #[test]

    fn insert_data_in_memory_and_find_it() -> Result<(), ErrService> {
        let (mut repo, user1, user2) = initialize_repo()?;

        assert!(repo.insert_data(&user1).is_ok());
        assert_eq!(repo.is_exist(&user1)?, true);
        assert_eq!(repo.is_exist(&user2)?, false);

        Ok(())
    }

    #[test]
    fn list_data_and_remove_data() -> Result<(), ErrService> {
        let (mut repo, user1, user2) = initialize_repo()?;

        assert!(repo.insert_data(&user1).is_ok());
        assert!(repo.list().is_ok());
        assert!(repo.remove_data(&user1).is_ok());
        assert!(repo.remove_data(&user2).is_err());

        Ok(())
    }
}
