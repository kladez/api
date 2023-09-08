macro_rules! env {
    ($name:literal) => {
        std::env::var($name).expect(concat!($name, " must be set"))
    };
    ($name:literal, $parser:ident) => {
        env!($name)
            .parse::<$parser>()
            .expect(concat!($name, " must be a ", stringify!($parser)))
    };
}

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub db_url: String,
    pub db_max_connections: u32,
    pub kafka_brokers: String,
    pub redis_url: String,
}

impl Config {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            host: env!("HOST"),
            db_url: env!("DATABASE_URL"),
            db_max_connections: env!("DATABASE_MAX_CONNECTIONS", u32),
            kafka_brokers: env!("KAFKA_BROKERS"),
            redis_url: env!("REDIS_URL"),
        }
    }
}
