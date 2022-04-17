use std::env;

use diesel::{PgConnection, Connection};
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL set!");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
