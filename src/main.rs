use std::{error::Error, net::SocketAddr};

use room_reservations::{app::build::build_app, config::Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let config = Config::init();
    let app = build_app(&config.database_url).await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
