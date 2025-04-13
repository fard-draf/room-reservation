#[cfg(test)]

mod test {
    use crate::{error::ErrService, tests::test_helpers::init_room_service};

    #[test]
    fn add_and_list_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service()?;

        let add_room_ok = room_service.add_room("El Palaccio");
        assert!(add_room_ok.is_ok(), "Add room should be ok");

        room_service.add_room("Dolce Note")?;
        room_service.add_room("Black room")?;

        let list_room_ok = room_service.list_rooms();
        assert!(list_room_ok.is_ok(), "List room should be ok");

        Ok(())
    }

    #[test]
    fn create_an_invalid_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service()?;

        let create_valid_room_trimed_ok =
            room_service.add_room("\t \t \t \t \t \t \t La Manche \t \t \t \t \t \t \t ");
        assert!(
            create_valid_room_trimed_ok.is_ok(),
            "Trimed room name should be ok"
        );

        let create_too_short_room_name_err = room_service.add_room("A");
        assert!(
            create_too_short_room_name_err.is_err(),
            "Too short room name should be an error"
        );

        let create_too_long_room_name_err =
            room_service.add_room("La chambre a coucher de madame de Bovary");
        assert!(
            create_too_long_room_name_err.is_err(),
            "Too long room name should be an error"
        );

        Ok(())
    }

    #[test]
    fn remove_an_existing_and_unexisting_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service()?;

        room_service.add_room("La Chambre Jaune")?;

        let exist_true = room_service.is_exist_room("La Chambre Jaune")?;
        assert_eq!(exist_true, true);

        let exist_false = room_service.is_exist_room("La Chambre Noire")?;
        assert_eq!(exist_false, false);

        let delete_existing_room_ok = room_service.remove_room("La Chambre Jaune");
        assert!(
            delete_existing_room_ok.is_ok(),
            "Remove existing room should be ok"
        );

        let delete_unexisting_room_err = room_service.remove_room("La Chambre Noire");
        assert!(
            delete_unexisting_room_err.is_err(),
            "Suppression has to fail"
        );

        Ok(())
    }

    #[test]
    fn find_existing_and_unexisting_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service()?;

        room_service.add_room("Palaccio")?;

        let find_existing_room = room_service.is_exist_room("Palaccio")?;
        assert_eq!(find_existing_room, true);

        let find_unexisting_room = room_service.is_exist_room("Ice Room")?;
        assert_eq!(find_unexisting_room, false);

        Ok(())
    }
}
