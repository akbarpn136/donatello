use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    nama: String,
    email: String
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/v1", web::get().to(|| HttpResponse::Ok().body("ini versi 1.")))
        .route("/v1/user", web::get().to(|| {
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
        }));
}