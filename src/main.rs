#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use rocket_contrib::json::Json;
use serde_json::Value as JsonValue;

use rocket::http::RawStr;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> &'static str {
    "assalamu'alaikum cekgu\n"
}

#[get("/nama/<name>/<umur>")]
fn nama_user(name: &RawStr, umur: i32) -> String {
    let pak_atau_mas = if umur > 60 {
        "mbah"
    } else if umur > 30 {
        "pak"
    } else if umur > 20 {
        "mas"
    } else {
        "dek"
    };

    format!("Hai, {} {} !\n", pak_atau_mas, name.as_str())
}

#[get("/download/<file..>")]
fn ke_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[derive(Serialize, Deserialize)]
struct Anggota {
    pub nama: String,
    pub email: String,
    pub alamat: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateAnggota {
    pub id: i64,
    pub nama: String,
    pub email: String,
    pub alamat: String,
}

#[derive(Serialize, Deserialize)]
struct IdQuery {
    pub id: i64,
}

#[derive(Serialize)]
struct ApiResult {
    pub result: Vec<models::Account>,
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref DB: Arc<Mutex<PgConnection>> = {
        let database_url = env::var("DATABASE_URL").expect("variable DATABASE_URL belum diset!");
        let conn =
            PgConnection::establish(&database_url).expect("gagal melakukan koneksi ke database");
        Arc::new(Mutex::new(conn))
    };
}

#[post("/register", format = "application/json", data = "<data>")]
fn register(data: Json<Anggota>) -> Json<JsonValue> {
    let conn = DB.lock().unwrap();

    let new_account = models::NewAccount {
        nama: &data.nama,
        email: &data.email,
        alamat: &data.alamat,
    };

    let account: models::Account = diesel::insert_into(schema::accounts::table)
        .values(&new_account)
        .get_result(&*conn)
        .expect("gagal insert akun ke dalam database");

    Json(
        serde_json::value::to_value(ApiResult {
            result: vec![account],
        })
        .expect("gagal meng-serialize data"),
    )
}

#[get("/anggota")]
fn daftar_anggota() -> Json<ApiResult> {
    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;

    let daftar = accounts
        .limit(10)
        .load::<models::Account>(&*conn)
        .expect("gagal query ke database");

    Json(ApiResult { result: daftar })
}

#[post("/anggota/update", data = "<data>")]
fn update(data: Json<UpdateAnggota>) -> Json<ApiResult> {
    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;

    let data_baru: models::Account = diesel::update(accounts.find(data.id))
        .set((
            nama.eq(&data.nama),
            email.eq(&data.email),
            alamat.eq(&data.alamat),
        ))
        .get_result::<models::Account>(&*conn)
        .expect("gagal update ke database");

    Json(ApiResult {
        result: vec![data_baru],
    })
}

#[post("/anggota/delete", data = "<data>")]
fn delete(data: Json<IdQuery>) -> String {
    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;

    diesel::delete(accounts.find(data.id))
        .execute(&*conn)
        .expect("gagal menghapus dari database");

    format!("Akun anggota id `{}` telah dihapus.\n", data.id)
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                nama_user,
                ke_file,
                register,
                daftar_anggota,
                update,
                delete
            ],
        )
        .launch();
}
