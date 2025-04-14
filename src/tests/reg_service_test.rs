#[cfg(test)]
mod test {
    use crate::{
        domain::*,
        error::{ErrBook, ErrService},
        tests::test_helpers::*,
    };

    #[test]
    fn add_and_list_book() -> Result<(), ErrService> {
        let mut reg_service = init_reg_service()?;
        let (room1, _room2) = default_rooms()?;
        let (user1, _user2) = default_users()?;

        let add_book_ok = reg_service.book_room(&room1, &user1, "01.02.52");
        assert!(add_book_ok.is_ok(), "Add book should be ok");

        Ok(())
    }

    #[test]
    fn valid_and_unvalid_date_format() -> Result<(), ErrBook> {
        assert!(BookDate::new("10.08.25").is_ok());
        assert!(BookDate::new("10/08/25").is_ok());

        assert!(BookDate::new("10/08/2025").is_err());
        assert!(BookDate::new("20.07.1987").is_err());
        assert!(BookDate::new("24.7.93").is_err());
        assert!(BookDate::new("21031962").is_err());

        Ok(())
    }

    #[test]
    fn print_book() -> Result<(), ErrService> {
        let mut reg_service = init_reg_service()?;
        let (room1, _room2) = default_rooms()?;
        let (user1, _user2) = default_users()?;

        reg_service.book_room(&room1, &user1, "20.12.26")?;

        assert!(reg_service.print_book().is_ok());

        Ok(())
    }

    #[test]
    fn book_an_already_booked_room() -> Result<(), ErrService> {
        let mut reg_service = init_reg_service()?;
        let (user1, user2) = default_users()?;
        let (room1, room2) = default_rooms()?;

        assert!(reg_service.book_room(&room1, &user1, "10.02.26").is_ok());
        assert!(reg_service.book_room(&room1, &user2, "10.02.26").is_err());

        Ok(())
    }
}
