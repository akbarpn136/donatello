use uuid::Uuid;
use bcrypt::hash;
use actix_web::{HttpResponse, web, Error};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::{SqliteConnection, insert_into, RunQueryDsl};
use actix_web::rt::blocking::BlockingError;

use crate::user::model::User;
use crate::schema::users::dsl::*;
use crate::user::interface::UserBaru;

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub async fn tambah_user(payload: web::Form<UserBaru>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let hashed_password = hash(payload.password.to_owned(), 5).map_err(|err| {
        HttpResponse::InternalServerError().body(err.to_string())
    })?;

    let user = web::block(move || -> Result<User, diesel::result::Error> {
        let user_baru = User {
            id: Uuid::new_v4().to_string(),
            nama: payload.nama.to_owned(),
            email: payload.email.to_owned(),
            password: hashed_password
        };

        insert_into(users)
            .values(&user_baru)
            .execute(&conn)?;

        Ok(user_baru)
    }).await.map_err(|err| {
        match err {
            BlockingError::Error(e) => HttpResponse::BadRequest().body(e.to_string()),
            BlockingError::Canceled => HttpResponse::InternalServerError().body("Cancled")
        }
    })?;

    Ok(HttpResponse::Created().json(user))
}