#[cfg(test)]

mod test {
    use crate::{domain::*, error::ErrService, infra::in_memo_repo::InMemoryRepo, repository::*};

    fn initialize_repo() -> Result<(InMemoryRepo<User>, User, User), ErrService> {
        let repo: InMemoryRepo<User> = InMemoryRepo::<User>::new();
        let user1: User = User::new("Walid")?;
        let user2 = User::new("Camille")?;

        Ok((repo, user1, user2))
    }

    #[test]

    fn insert_data_in_memory_and_find_it() -> Result<(), ErrService> {
        let (mut repo, user1, user2) = initialize_repo()?;

        assert!(repo.insert_data(&user1).is_ok());
        assert_eq!(repo.is_empty(&user1)?, true);
        assert_eq!(repo.is_empty(&user2)?, false);

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
