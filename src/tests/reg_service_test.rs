#[cfg(test)]
mod test {
    use crate::{
        domain::*,
        infra::{
            in_memo_repo::InMemoryRepo,
            reg_service::RegService,
        },
    };
    use std::error::Error;

    fn setup_environment()
    -> Result<(Room, Room, User, User, RegService<InMemoryRepo<Book>>), Box<dyn Error>> {
        let room1 = Room::new("Suite Royale")?;
        let room2 = Room::new("Suite Nuptiale")?;

        let user1 = User::new("Boris")?;
        let user2 = User::new("Aline")?;

        let repo = InMemoryRepo::new();
        let reg_service = RegService::new(repo);

        Ok((room1, room2, user1, user2, reg_service))
    }

    #[test]
    fn add_and_list_book() -> Result<(), Box<dyn Error>> {
        let (room1, _room2, user1, _user2, mut reg_service) = setup_environment()?;

        let add_book_ok = reg_service.book_room(&room1, &user1, "01.02.52");
        assert!(add_book_ok.is_ok(), "Add book should be ok");

        Ok(())
    }

    #[test]
    fn valid_and_unvalid_date_format() -> Result<(), Box<dyn Error>> {
        assert!(BookDate::new("10.08.25").is_ok());
        assert!(BookDate::new("10/08/25").is_ok());

        assert!(BookDate::new("10/08/2025").is_err());
        assert!(BookDate::new("20.07.1987").is_err());
        assert!(BookDate::new("24.7.93").is_err());
        assert!(BookDate::new("21031962").is_err());

        Ok(())
    }

    // #[test]
}
