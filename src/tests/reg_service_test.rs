#[cfg(test)]
mod test {
    use crate::{
        domain::BookDate,
        error::{ErrBook, ErrService},
        tests::test_helpers::{default_rooms, default_users, init_reg_service},
    };

    #[tokio::test]

    async fn add_and_list_book() -> Result<(), ErrService> {
        let mut reg_service = init_reg_service().await?;
        let (room1, _room2) = default_rooms().await?;
        let (user1, _user2) = default_users().await?;

        let add_book_ok = reg_service.book_room(&room1, &user1, "01.02.52");
        assert!(add_book_ok.is_ok(), "Add book should be ok");

        Ok(())
    }

    #[tokio::test]

    async fn valid_and_unvalid_date_format() -> Result<(), ErrBook> {
        assert!(BookDate::new("10.08.25").is_ok());
        assert!(BookDate::new("10/08/25").is_ok());

        assert!(BookDate::new("100825").is_err());
        assert!(BookDate::new("10/08/2025").is_err());
        assert!(BookDate::new("20.07.1987").is_err());
        assert!(BookDate::new("24.7.93").is_err());
        assert!(BookDate::new("21031962").is_err());

        Ok(())
    }

    #[tokio::test]
    async fn print_book() -> Result<(), ErrService> {
        let mut reg_service = init_reg_service().await?;
        let (room1, _room2) = default_rooms().await?;
        let (user1, _user2) = default_users().await?;

        reg_service.book_room(&room1, &user1, "20.12.26")?;

        assert!(reg_service.print_book().is_ok());

        Ok(())
    }

    #[tokio::test]

    async fn book_an_already_booked_room() -> Result<(), ErrService> {
        let mut reg_service = init_reg_service().await?;
        let (user1, user2) = default_users().await?;
        let (room1, room2) = default_rooms().await?;

        assert!(reg_service.book_room(&room1, &user1, "10.02.26").is_ok());
        assert!(reg_service.book_room(&room1, &user2, "10.02.26").is_err());
        assert!(reg_service.book_room(&room2, &user2, "10.02.26").is_ok());

        Ok(())
    }
}
