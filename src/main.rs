mod routers;

use std::io;
use std::env;
use actix_web::{HttpServer, App, web, HttpResponse};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let address = format!("{}:{}",
                          env::var("APP_HOST").expect("variabel HOST perlu didefiniskan"),
                          env::var("APP_PORT").expect("variabel PORT perlu didefiniskan"));
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| HttpResponse::Ok().body("Halaman ini dikosongkan")))
            .configure(routers::config)
    }).bind(address).unwrap();

    println!("Menjalankan server...");
    server.run().await
}
