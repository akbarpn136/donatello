use actix_web::{web, HttpResponse};

use crate::user::model::User;
use crate::user::interface::Info;
use crate::state::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(|state: web::Data<AppState>| {
            let app_name = &state.app_name;
            let user = User {
                nama: String::from("Fulan"),
                email: String::from("fulan@gmail.com")
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
        }))
        .route("/", web::post().to(|payload: web::Form<User>,
                                    state: web::Data<AppState>| {
            let app_name = &state.app_name;
            HttpResponse::Created().body(
                format!("User {} denagn email {} berhasil dibuat melalui {}", payload.nama,
                        payload.email, app_name)
            )
        }));
}
