use crate::helpers::require_env_var_string;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub bind_address: std::net::SocketAddr,
}

impl Config {
    pub fn new() -> Self {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            require_env_var_string("DATABASE_USER"),
            require_env_var_string("DATABASE_PASSWORD"),
            require_env_var_string("DATABASE_HOST"),
            require_env_var_string("DATABASE_PORT"),
            require_env_var_string("DATABASE_NAME"),
        );

        let bind_address: std::net::SocketAddr = format!(
            "{}:{}",
            require_env_var_string("BIND_HOST"),
            require_env_var_string("BIND_PORT"),
        )
        .parse()
        .map_err(|e| format!("Failed to parse BIND_HOST and BIND_PORT concatenation: {e:?}"))
        .unwrap();

        Self {
            database_url,
            bind_address,
        }
    }
}
