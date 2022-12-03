use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use backend::DbPool;
use backend::{establish_connection, models::*};
use diesel::prelude::*;

struct AppState {
    pub pool: DbPool,
}

#[get("/products")]
async fn get_products(data: web::Data<AppState>) -> impl Responder {
    use backend::schema::products::dsl::*;

    let connection = &mut data.pool.get().unwrap();

    let result: Vec<Product> = products
        .limit(5)
        .load::<Product>(connection)
        .expect("Error loading products");

    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO Add logging tool
    let app_data = web::Data::new(AppState {
        pool: establish_connection(),
    });

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(get_products))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
