#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use std::env;
use std::sync::Arc;

mod handler;
mod models;
mod resolver;
mod schema;

use handler::*;

use crate::resolver::root::*;

pub struct AppState {
    schema: Arc<Schema>,
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();
    let schema = Arc::new(create_schema());
    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                schema: schema.clone(),
                pool: pool.clone(),
            })
            .service(graphql)
            .service(graphiql)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
