use std::env;

use diesel::{PgConnection, Connection};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL set!");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
