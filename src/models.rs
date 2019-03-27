use crate::schema::*;

use super::chrono;
use chrono::*;

#[derive(Queryable, Serialize)]
pub struct AccessToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
}

#[derive(Insertable)]
#[table_name = "access_tokens"]
pub struct NewAccessToken<'a> {
    pub user_id: i64,
    pub token: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct Account {
    pub id: i64,
    pub nama: String,
    pub email: String,
    pub alamat: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub nama: &'a str,
    pub email: &'a str,
    pub alamat: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct Article {
    pub id: i64,
    pub judul: String,
    pub konten: String,
    pub waktu: NaiveDateTime,
    pub penulis: String,
}

#[derive(Insertable, Queryable)]
#[table_name = "articles"]
pub struct NewArticle<'a> {
    pub judul: &'a str,
    pub konten: &'a str,
    pub waktu: chrono::NaiveDateTime,
    pub penulis: &'a str,
}
