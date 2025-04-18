#[cfg(test)]

mod test {
    use crate::{app::state::AppState, error::ErrService, tests::test_helpers::init_room_service};

    #[tokio::test]

    async fn add_and_list_room() -> Result<(), ErrService> {
        let service = 

        assert!(room_service.("El Palaccio").await.is_ok());

        room_service.add_room("Dolce Note").await?;
        room_service.add_room("Black room").await?;

        assert!(room_service.list_rooms().await.is_ok());

        Ok(())
    }

    #[tokio::test]

    async fn create_an_invalid_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service().await?;

        assert!(
            room_service
                .add_room("\t \t \t \t \t \t \t La Manche \t \t \t \t \t \t \t ")
                .await
                .is_ok()
        );
        assert!(room_service.add_room("A").await.is_err());
        assert!(
            room_service
                .add_room("La chambre a coucher de madame de Bovary")
                .await
                .is_err()
        );

        Ok(())
    }

    #[tokio::test]

    async fn remove_an_existing_and_unexisting_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service().await?;
        room_service.add_room("La Chambre Jaune").await?;

        assert_eq!(room_service.is_exist_room("La Chambre Jaune").await?, true);
        assert_eq!(room_service.is_exist_room("La Chambre Noire").await?, false);

        assert!(room_service.remove_room("La Chambre Jaune").await.is_ok());

        assert!(room_service.remove_room("La Chambre Noire").await.is_err());

        Ok(())
    }

    #[tokio::test]

    async fn find_existing_and_unexisting_room() -> Result<(), ErrService> {
        let mut room_service = init_room_service().await?;
        room_service.add_room("Palaccio").await?;

        assert_eq!(room_service.is_exist_room("Palaccio").await?, true);
        assert_eq!(room_service.is_exist_room("Ice Room").await?, false);

        Ok(())
    }
}
