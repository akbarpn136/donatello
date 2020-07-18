use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    nama: String,
    email: String
}

#[derive(Deserialize)]
struct Info {
    kelompok: String
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/v1/", web::get().to(|| HttpResponse::Ok().body("ini versi 1.")))
        .route("/v1/user/", web::get().to(|| {
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
        .route("/v1/user/{id}/", web::get().to(|id: web::Path<String>, informasi: Option<web::Query<Info>>| {
            let body = if informasi.is_none() {
                format!("User id: {}", id)
            } else {
                format!("User id: {} dengan kelompok {}", id, informasi.unwrap().kelompok)
            };

            HttpResponse::Ok().body(body)
        }))
        .route("/v1/user/", web::post().to(|payload: web::Form<User>| {
            HttpResponse::Created().body(format!("User {} denagn email {} berhasil dibuat", payload.nama, payload.email))
        }));
}