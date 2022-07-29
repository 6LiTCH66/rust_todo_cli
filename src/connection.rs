use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn database_connection() -> PgConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set!");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
