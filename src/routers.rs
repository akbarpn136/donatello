use actix_web::{web, HttpResponse};
use crate::user::routers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/v1/", web::get().to(|| HttpResponse::Ok().body("ini versi 1.")))
        .service(
            web::scope("/v1/user").configure(routers::config)
        );
}