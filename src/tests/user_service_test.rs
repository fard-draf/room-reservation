#[cfg(test)]
mod test {
    use crate::{
        domain::*,
        error::{ErrDB, ErrService},
        tests::test_helpers::{default_users, init_user_service},
    };

    #[test]
    fn add_and_list_user() -> Result<(), ErrService> {
        let user_service = init_user_service()?;
        let (user1, user2) = default_users()?;

        let users = user_service.list_users()?;

        assert_eq!(users.len(), 2);
        assert_eq!(users[0], user1);
        assert_eq!(users[1], user2);

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
        let mut user_service = init_user_service()?;

        user_service.add_user("Boris")?;

        assert!(user_service.is_exist_user("Boris").is_ok());
        assert!(user_service.is_exist_user("Joris").is_err());

        assert!(user_service.remove_user("Boris").is_ok());
        assert!(user_service.remove_user("Joris").is_err());

        Ok(())
    }
}
