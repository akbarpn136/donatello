#[macro_use]
extern crate diesel;

mod state;
mod user;
mod routers;
mod schema;

use std::io;
use std::env;
use actix_web::{HttpServer, App, web, middleware, HttpResponse};

use crate::state::AppState;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let manager = ConnectionManager::<SqliteConnection>::new("donatello.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    let address = format!("{}:{}",
                          env::var("APP_HOST").expect("variabel HOST perlu didefinisikan"),
                          env::var("APP_PORT").expect("variabel PORT perlu didefinisikan"));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .data(AppState {
                app_name: "donatello".to_string()
            })
            .data(pool.clone())
            .route("/", web::get().to(|| HttpResponse::Ok().body("Halaman ini dikosongkan")))
            .configure(routers::config)
    }).bind(address).unwrap();

    println!("Menjalankan server...");
    server.run().await
}
