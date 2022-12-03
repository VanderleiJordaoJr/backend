pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use r2d2::{Pool};
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    
	let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}
