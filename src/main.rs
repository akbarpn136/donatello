mod state;
mod user;
mod routers;

use std::io;
use std::env;
use actix_web::{HttpServer, App, web, middleware, HttpResponse};

use crate::state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let address = format!("{}:{}",
                          env::var("APP_HOST").expect("variabel HOST perlu didefinisikan"),
                          env::var("APP_PORT").expect("variabel PORT perlu didefinisikan"));
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath)
            .data(AppState {
                app_name: "donatello".to_string()
            })
            .route("/", web::get().to(|| HttpResponse::Ok().body("Halaman ini dikosongkan")))
            .configure(routers::config)
    }).bind(address).unwrap();

    println!("Menjalankan server...");
    server.run().await
}
