// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenv::dotenv;
// use std::env;

// use chrono::prelude::*;

use crate::schema::{accounts, articles};

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

use chrono::NaiveDateTime;
#[derive(Queryable, Serialize)]
pub struct Article {
    pub id: i64,
    pub judul: String,
    pub konten: String,
    pub waktu: String,
    pub penulis: String,
}

#[derive(Insertable, Queryable)]
#[table_name = "articles"]
pub struct NewArticle<'a> {
    pub judul: &'a str,
    pub konten: &'a str,
    pub penulis: &'a str,
}
