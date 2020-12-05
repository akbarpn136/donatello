use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub kelompok: String
}

#[derive(Deserialize)]
pub struct UserBaru {
    pub nama: String,
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct UbahUser {
    pub nama: String,
}
