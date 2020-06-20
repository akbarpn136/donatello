mod routers;

use std::io;
use actix_web::{HttpServer, App, web, HttpResponse};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| HttpResponse::Ok().body("Halaman ini dikosongkan")))
            .configure(routers::config)
    }).bind("localhost:8080").unwrap();

    println!("Menjalankan server...");
    server.run().await
}
