use actix_web::{web, HttpResponse};

use crate::user::model::User;
use crate::user::interface::Info;
use crate::state::AppState;
use crate::user::handlers::tambah_user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(tambah_user))
        .route("/", web::get().to(|state: web::Data<AppState>| {
            let app_name = &state.app_name;
            let user = User {
                id: "".to_string(),
                nama: String::from("Fulan"),
                email: String::from("fulan@gmail.com"),
                password: "".to_string()
            };

            let user_tostring = serde_json::to_string(&user).unwrap();
            let user_fromstring: User = serde_json::from_str(&user_tostring).unwrap();

            println!("{:?}", user);
            println!("{}", user_tostring);
            println!("{:?}", user_fromstring);
            println!("Nama aplikasi ini: {}", app_name);

            HttpResponse::Ok().body("ini versi 1 untuk user")
        }))
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
