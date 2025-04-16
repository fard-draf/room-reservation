use std::{error::Error, net::SocketAddr};

use room_reservations::app::build::build_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_url = "postgres://rust_tester:Lvetiflb2020@192.168.2.214:5432/rust_test_db";

    let app = build_app(database_url).await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
