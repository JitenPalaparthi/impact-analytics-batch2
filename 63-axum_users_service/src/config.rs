use std::env;
use std::net::SocketAddr;

pub struct Config {
    pub database_url: String,
    pub server_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/usersdb".to_string());

        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port: u16 = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        let addr: SocketAddr = format!("{}:{}", host, port)
            .parse()
            .expect("valid socket address");

        Self {
            database_url,
            server_addr: addr,
        }
    }
}
