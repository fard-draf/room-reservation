#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url =
            std::env::var("DATABASE_URL").expect("Please enter a valid database address.");

        Config { database_url }
    }
}
