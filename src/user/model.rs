use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
pub struct User {
    pub id: String,
    pub nama: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Klaim {
    pub sub: String,
    pub iat: i64,
    pub exp: i64
}