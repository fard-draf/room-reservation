#[cfg(test)]
mod test {
    use crate::{
        domain::*,
        error::{ErrDB, ErrService},
        infra::{in_memo_repo::InMemoryRepo, user_service::*},
    };
    use std::error::Error;

    fn inialize_repo() -> Result<UserService<InMemoryRepo<User>>, ErrDB> {
        let repo_user: InMemoryRepo<User> = InMemoryRepo::new();
        Ok(UserService::new(repo_user))
    }

    #[test]
    fn add_and_list_user() -> Result<(), ErrService> {
        let mut user_service = inialize_repo()?;

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

    #[test]
    fn create_an_invalid_user_too_short_or_too_long() -> Result<(), ErrService> {
        assert!(UserName::new("\t \t \t \t Daniel \t \t \t ").is_ok());
        assert!(UserName::new("A").is_err());
        assert!(
            UserName::new("Jean-Edouard De la Portaillere de la Manchada de la Cabesa").is_err()
        );

        Ok(())
    }

    #[test]
    fn remove_an_existing_and_unexisting_user() -> Result<(), ErrService> {
        let mut user_service = inialize_repo()?;

        user_service.add_user("Boris")?;

        assert!(user_service.is_exist_user("Boris").is_ok());
        assert!(user_service.is_exist_user("Joris").is_err());

        assert!(user_service.remove_user("Boris").is_ok());
        assert!(user_service.remove_user("Joris").is_err());

        Ok(())
    }
}
