mod state;
mod user;
mod routers;

use std::io;
use std::env;
use actix_web::{HttpServer, App, web, middleware, HttpResponse};

use crate::state::AppState;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let manager = SqliteConnectionManager::file("donatello.db");
    let pool = Pool::new(manager).unwrap();

    let address = format!("{}:{}",
                          env::var("APP_HOST").expect("variabel HOST perlu didefinisikan"),
                          env::var("APP_PORT").expect("variabel PORT perlu didefinisikan"));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath)
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
