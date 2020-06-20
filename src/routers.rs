use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1", web::get().to(|| HttpResponse::Ok().body("ini versi 1.")));
}