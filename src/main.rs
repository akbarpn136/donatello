mod models;
mod errors;

use actix_web::{HttpServer, App, web, HttpResponse};
use crate::models::BelimbingError;


#[actix_web::main]
async fn main() -> Result<(), BelimbingError> {
    // println!("{:?}", concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

    let address = "0.0.0.0:8080";
    let server = HttpServer::new(|| {
        App::new().service(
            web::resource("/").route(web::get().to(|| HttpResponse::Ok().body("OKE")))
        )
    });

    println!("Menjalankan aplikasi di {}", address);

    Ok(server.bind(address)?.run().await?)
}
