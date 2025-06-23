use libsql::{Connection, Database};
use std::env;
use dotenvy::dotenv;

pub async fn db_connection() -> Connection {
    dotenv().expect(".env file not found");

    let db_url = env::var("DATABASE_URL").unwrap();

    let db = Database::open(db_url).unwrap();

    db.connect().unwrap()
}

