use std::error::Error;

use crate::{domain::*, infra::{in_memo_repo::*, room_service::{self, RoomService}, user_service::*}, repository::DBRepository};


// TESTS infra/user_service.rs

#[cfg(test)]
mod test {} 

    #[test] 
    fn add_and_list_user() -> Result<(), Box<dyn Error>> {
        let repo_user: InMemoryRepo<User> = InMemoryRepo::new();
        let mut user_service: UserService<InMemoryRepo<User>> = UserService::new(repo_user);


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
    fn remove_an_existing_and_unexisting_user() -> Result<(), Box<dyn Error>> {
        let repo_user: InMemoryRepo<User> = InMemoryRepo::new();
        let mut user_service: UserService<InMemoryRepo<User>> = UserService::new(repo_user);

        user_service.add_user("Boris")?;

        let exist_true = user_service.is_exist_user("Boris")?;
        assert_eq!(exist_true, true);
        
        let exist_false = user_service.is_exist_user("Joris")?;
        assert_eq!(exist_false, false);
        
        let delete_user_ok = user_service.remove_user("Boris");
        assert!(delete_user_ok.is_ok(), "Suppression has to succed");

        let delete_user_err = user_service.remove_user("Joris");
        assert!(delete_user_err.is_err(), "Suppression has to fail");

        
        Ok(())
     
    }

// TEST room_service.rs
    
    #[test]
    fn add_and_list_room()-> Result<(), Box<dyn Error>> {
        let repo_room: InMemoryRepo<Room> = InMemoryRepo::new();
        let mut room_service: RoomService<InMemoryRepo<Room>> = RoomService::new(repo_room);

        let add_a_room_ok = room_service.add_room("Suite Royale");
        assert!(add_a_room_ok.is_ok(), "Add a room should be ok");
        room_service.add_room("Etape nocturne")?;
        room_service.add_room("Dolce Note")?;
        room_service.add_room("Black room")?;

        let list_room_ok = room_service.list_rooms();
        assert!(list_room_ok.is_ok(), "List room should be ok");


        Ok(())
    }

    #[test]
    fn remove_existing_unexisting_room() -> Result<(), Box<dyn Error>> {
        let repo_room: InMemoryRepo<Room> = InMemoryRepo::new();
        let mut room_service: RoomService<InMemoryRepo<Room>> = RoomService::new(repo_room);

        room_service.add_room("Suite Royale")?;

        let remove_existing_room_ok = room_service.remove_room("Suite Royale");
        assert!(remove_existing_room_ok.is_ok(), "Remove an existing room should be ok");
        
        let remove_unexisting_room_err = room_service.remove_room("Forest Cabin");
        assert!(remove_unexisting_room_err.is_err(), "Remove an unexisting room should be an error");
        Ok(())
    }

    #[test]
    fn find_existing_and_unexisting_room() -> Result<(), Box<dyn Error>> {
        let repo_room: InMemoryRepo<Room> = InMemoryRepo::new();
        let mut room_service: RoomService<InMemoryRepo<Room>> = RoomService::new(repo_room);

        room_service.add_room("Palaccio")?;

        let find_existing_room = room_service.is_exist_room("Palaccio")?;
        assert_eq!(find_existing_room, true);

        let find_unexisting_room = room_service.is_exist_room("Ice Room")?;
        assert_eq!(find_unexisting_room, false);



        Ok(())

    }
    