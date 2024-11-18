mod database;
mod server;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::sync::{Arc, Mutex};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let _ = establish_connection();

    server::income_packet::Test::unpack(&[0, 0, 0, 0, 0, 0, 0, 0]);

    server::tcp::Server::create_server(
        Arc::new(server::tcp::Server {
            db: Arc::new(Mutex::new(establish_connection())),
        }),
        "0.0.0.0",
        "1973",
    );
}
