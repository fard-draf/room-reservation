#[cfg(test)]

mod test {
    use crate::{error::ErrService, tests::test_helpers::init_room_service};

    #[tokio::test]

    async fn add_and_list_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service().await?;

        assert!(room_service.add_room("El Palaccio").is_ok());

        room_service.add_room("Dolce Note")?;
        room_service.add_room("Black room")?;

        assert!(room_service.list_rooms().is_ok());

        Ok(())
    }

    #[tokio::test]

    async fn create_an_invalid_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service().await?;

        assert!(
            room_service
                .add_room("\t \t \t \t \t \t \t La Manche \t \t \t \t \t \t \t ")
                .is_ok()
        );
        assert!(room_service.add_room("A").is_err());
        assert!(
            room_service
                .add_room("La chambre a coucher de madame de Bovary")
                .is_err()
        );

        Ok(())
    }

    #[tokio::test]

    async fn remove_an_existing_and_unexisting_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service().await?;
        room_service.add_room("La Chambre Jaune")?;

        assert_eq!(room_service.is_exist_room("La Chambre Jaune")?, true);
        assert_eq!(room_service.is_exist_room("La Chambre Noire")?, false);

        assert!(room_service.remove_room("La Chambre Jaune").is_ok());

        assert!(room_service.remove_room("La Chambre Noire").is_err());

        Ok(())
    }

    #[tokio::test]

    async fn find_existing_and_unexisting_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service().await?;
        room_service.add_room("Palaccio")?;

        assert_eq!(room_service.is_exist_room("Palaccio")?, true);
        assert_eq!(room_service.is_exist_room("Ice Room")?, false);

        Ok(())
    }
}
