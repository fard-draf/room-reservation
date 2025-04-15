use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::{error::Error, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

use room_reservations::*;

// #[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // let database_url = "postgres://rust_tester:Lvetiflb2020@192.168.2.214:5432/rust_test_db";
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(database_url)
    //     .await?;

    // let repo_user = DBClient::new(pool);

    // let user_service: UserService<DBClient> = UserService::new(repo_user);
    // let shared_user_service: SharedUserService<DBClient> = Arc::new(Mutex::new(user_service));

    // let app = Router::new().merge(user_routes(shared_user_service.clone()));

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("Server running on http://{}", addr);

    // // Nouvelle API d'Axum
    // axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    // Ok(())
}
