use uuid::Uuid;
use bcrypt::{hash, verify};
use actix_web::{HttpResponse, web, Error, ResponseError};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::{SqliteConnection, insert_into, RunQueryDsl, QueryDsl, ExpressionMethods};

use crate::user::model::{User, Klaim};
use crate::schema::users::dsl::*;
use crate::user::dto::{UserBaru, UbahUser, LoginUser};
use std::env;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};

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

pub async fn ubah_user_id(
    uid: web::Path<Uuid>,
    payload: web::Form<UbahUser>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let user = web::block(move || -> Result<User, diesel::result::Error> {
        diesel::update(users.filter(id.eq(uid.to_string())))
            .set(nama.eq(payload.0.nama))
            .execute(&conn)
            .unwrap();

        let user_id = users
            .filter(id.eq(uid.to_string()))
            .first::<User>(&conn)?;

        Ok(user_id)
    }).await.map_err(|err| err.error_response())?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn hapus_user_id(
    uid: web::Path<Uuid>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let user = web::block(move || -> Result<usize, diesel::result::Error> {
        let count = diesel::delete(
            users.filter(
                id.eq(uid.to_string())
            )
        ).execute(&conn).unwrap();

        Ok(count)
    }).await
        .map_err(|err| err.error_response())?;

    Ok(HttpResponse::Ok().body(format!("Jumlah user yang dihapus {}", user)))
}

pub async fn login(
    payload: web::Form<LoginUser>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let mail = payload.0.email;
    let pwd = payload.0.password;

    let user_id = web::block(move || -> Result<Option<User>, diesel::result::Error> {
        let usr = users.filter(email.eq(mail))
            .first::<User>(&conn)?;

        Ok(Some(usr))
    }).await.map_err(|err| err.error_response())?;

    let err_message = Error::from(
        HttpResponse::Unauthorized().body("Email/Password tidak ditemukan.")
    );

    if let Some(usr) = user_id {
        let validkah = verify(pwd, &usr.password).unwrap();

        if validkah {
            let secret = env::var("APP_SECRET").unwrap();
            let iat = Utc::now();
            let exp = iat + Duration::days(7);
            let klaim = Klaim {
                sub: usr.nama,
                iat: iat.timestamp_nanos(),
                exp: exp.timestamp_nanos()
            };

            let token = encode(
                &Header::default(),
                &klaim,
                &EncodingKey::from_secret(secret.as_bytes())
            ).unwrap();

            Ok(HttpResponse::Ok().body(token))
        } else {
            Err(err_message)
        }
    } else {
        Err(err_message)
    }
}
