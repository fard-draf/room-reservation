use std::net::SocketAddr;

use room_reservations::{app::build::build_app, config::Config, error::ErrService};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), ErrService> {
    let subscriber = FmtSubscriber::builder().with_env_filter("info").finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    dotenv::dotenv().ok();

    let config = Config::init();
    let app = build_app(&config.database_url).await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
