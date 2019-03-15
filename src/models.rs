// #[macro_use]
// use diesel::prelude::*;
// use diesel::pg::PgConnection;
// use dotenv::dotenv;
// use std::env;

use crate::schema::accounts;
use crate::schema::articles;

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

extern crate chrono;
extern crate chrono_tz;
use chrono::{NaiveDateTime};
// use std::time::SystemTime;
// use chrono_tz::Tz;
// use chrono_tz::UTC;

// let tz: Tz = "Antarctica/South_Pole".parse().unwrap();
// let dt = tz.ymd(2016, 10, 22).and_hms(12, 0, 0);
// let utc = dt.with_timezone(&UTC);
// assert_eq!(utc.to_string(), "2016-10-21 23:00:00 UTC");

// use chrono::*;
// use diesel::sql_types::Timestamp;

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
    pub penulis: &'a str,
}
