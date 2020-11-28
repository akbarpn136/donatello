use uuid::Uuid;
use bcrypt::hash;
use actix_web::{HttpResponse, web, Error, ResponseError};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::{SqliteConnection, insert_into, RunQueryDsl, QueryDsl, ExpressionMethods};

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
        err.error_response()
    })?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn ambil_user(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let user = web::block(move || -> Result<Vec<User>, diesel::result::Error> {
        let lihat_user = users.load::<User>(&conn)?;

        Ok(lihat_user)
    }).await.map_err(|e| {
        e.error_response()
    })?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn ambil_user_id(uid: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let user = web::block(move || -> Result<Option<User>, diesel::result::Error> {
        let user_id = users
            .filter(id.eq(uid.to_string()))
            .first::<User>(&conn)?;

        Ok(Some(user_id))
    }).await.map_err(|err| {
        err.error_response()
    })?;

    Ok(HttpResponse::Ok().json(user))
}
