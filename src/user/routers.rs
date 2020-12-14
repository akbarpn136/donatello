use actix_web::web;
use crate::user::handlers::{
    tambah_user, ambil_user,
    ambil_user_id,
    ubah_user_id,
    hapus_user_id
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(tambah_user))
        .route("/", web::get().to(ambil_user))
        .route("/{id}/", web::get().to(ambil_user_id))
        .route("/{id}/", web::put().to(ubah_user_id))
        .route("/{id}/", web::delete().to(hapus_user_id));
}
