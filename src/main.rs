use std::{net::SocketAddr, sync::Arc, error::Error};
use room_reservations::infra::user_service;
use tokio::sync::Mutex;
use axum::Router;

use room_reservations::*;


use room_reservations::{
    domain::{Book, Room, User},
    error::ErrService,
    infra::{
        in_memo_repo::InMemoryRepo, 
        reg_service::RegService, 
        room_service::RoomService,
        user_service::UserService},
    routes::user_routes::user_routes,
    handlers::user_handlers::SharedUserService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {

    // let repo_room: InMemoryRepo<Room> = InMemoryRepo::new();
    // let mut _room_service: RoomService<InMemoryRepo<Room>> = RoomService::new(repo_room);

    // let repo_reg: InMemoryRepo<Book> = InMemoryRepo::new();
    // let mut _reg_service: RegService<InMemoryRepo<Book>> = RegService::new(repo_reg);

    let repo_user: InMemoryRepo<User> = InMemoryRepo::new();
    let mut user_service: UserService<InMemoryRepo<User>> = UserService::new(repo_user);
    let shared_user_service: SharedUserService<InMemoryRepo<User>> = Arc::new(Mutex::new(user_service));

    let app = Router::new()
    .merge(user_routes(shared_user_service.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    // Nouvelle API d'Axum
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await?;
    
    Ok(())
    
    
    
}
