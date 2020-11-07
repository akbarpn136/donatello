use actix_web::{web, HttpResponse};

use crate::user::interface::Info;
use crate::state::AppState;
use crate::user::handlers::{tambah_user, ambil_user};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(tambah_user))
        .route("/", web::get().to(ambil_user))
        .route("/{id}/", web::get().to(|id: web::Path<String>,
                                        informasi: Option<web::Query<Info>>,
                                        state: web::Data<AppState>| {
            let app_name = &state.app_name;
            let body = if informasi.is_none() {
                format!("User id: {} di aplikasi {}", id, app_name)
            } else {
                format!("User id: {} dengan kelompok {} di aplikasi {}", id,
                        informasi.unwrap().kelompok,
                        app_name)
            };

            HttpResponse::Ok().body(body)
        }));
}
