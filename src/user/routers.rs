use actix_web::{web, HttpResponse};

use crate::user::model::User;
use crate::user::interface::Info;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(|| {
            let user = User {
                nama: String::from("Fulan"),
                email: String::from("fulan@gmail.com")
            };

            let user_tostring = serde_json::to_string(&user).unwrap();
            let user_fromstring: User = serde_json::from_str(&user_tostring).unwrap();

            println!("{:?}", user);
            println!("{}", user_tostring);
            println!("{:?}", user_fromstring);

            HttpResponse::Ok().body("ini versi 1 untuk user")
        }))
        .route("/{id}/", web::get().to(|id: web::Path<String>, informasi: Option<web::Query<Info>>| {
            let body = if informasi.is_none() {
                format!("User id: {}", id)
            } else {
                format!("User id: {} dengan kelompok {}", id, informasi.unwrap().kelompok)
            };

            HttpResponse::Ok().body(body)
        }))
        .route("/", web::post().to(|payload: web::Form<User>| {
            HttpResponse::Created().body(format!("User {} denagn email {} berhasil dibuat", payload.nama, payload.email))
        }));
}
