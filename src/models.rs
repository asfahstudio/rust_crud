// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenv::dotenv;
// use std::env;

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

// use chrono::offset::TimeZone;
// use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
// use chrono_tz::Asia::Jakarta;

// let london_time = London.ymd(2016, 3, 18).and_hms(3, 0, 0);
// let ny_time = london_time.with_timezone(&New_York);
// assert_eq!(ny_time, New_York.ymd(2016, 3, 17).and_hms(23, 0, 0));
// use std::time::SystemTime;
use chrono::*;

use super::chrono;

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
