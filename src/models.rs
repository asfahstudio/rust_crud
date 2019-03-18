// #[macro_use]
// use diesel::prelude::*;
// use diesel::pg::PgConnection;
// use dotenv::dotenv;
// use std::env;


use crate::schema::accounts;
use crate::schema::articles;
use chrono::*;
use super::chrono;

#[derive(Queryable, Serialize)]
pub struct Account {
    pub id: i64,
    pub nama: String,
    pub email: String,
    pub alamat: String,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub nama: &'a str,
    pub email: &'a str,
    pub alamat: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct Article {
    pub id: i64,
    pub judul: String,
    pub konten: String,
    pub waktu: NaiveDateTime,
    pub penulis: String,
}

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "articles"]
pub struct NewArticle<'a> {
    pub judul: &'a str,
    pub konten: &'a str,
    pub waktu: chrono::NaiveDateTime,
    pub penulis: &'a str,
}
