use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use r2d2::PooledConnection;
use std::env;

fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let conn = pool.clone().get().unwrap();
    conn
}

struct AppState {
    database_connection: PooledConnection<ConnectionManager<PgConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().app_data(web::Data::new(AppState {
            database_connection: establish_connection(),
        }))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
