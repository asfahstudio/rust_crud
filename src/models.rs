use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::schema::{accounts,article};

#[derive(Queryable, Serialize)]
pub struct Account {
    pub id:i64,
    pub nama:String,
    pub email:String,
    pub alamat:String
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub nama:&'a str,
    pub email:&'a str,
    pub alamat:&'a str
}

#[derive(Queryable,Serialize)]
pub struct Artikel {
    pub id:i64,
    pub judul:String,
    pub konten:String,
    pub penulis:String,
    pub published:String,
}

#[derive(Insertable)]
#[table_name="article"]
pub struct NewArtikel<'a> {
    pub judul:&'a str,
    pub konten:&'a str,
    pub penulis:&'a str,
}