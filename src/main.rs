#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;
extern crate rocket_contrib;

// #[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate diesel;
extern crate dotenv;

// #[macro_use]
extern crate chrono;
extern crate chrono_tz;

mod models;
mod schema;

use chrono::prelude::Local;

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

// serde table accounts
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
// end serde table account

// serde table articles
#[derive(Serialize, Deserialize)]
struct AddArticle {
    pub judul: String,
    pub konten: String,
    pub penulis: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateArticle {
    pub id: i64,
    pub judul: String,
    pub konten: String,
    pub penulis: String,
}

#[derive(Serialize, Deserialize)]
struct IdQueryArticle {
    pub id: i64,
}

#[derive(Serialize)]
struct ApiResultArticle {
    pub result: Vec<models::Article>,
}
// end serde table aarticles

// use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref DB: Arc<Mutex<PgConnection>> = {
        let database_url = env::var("DATABASE_URL").expect("variable DATABASE_URL belum diset!");
        let conn =
            PgConnection::establish(&database_url).expect("gagal melakukan koneksi ke database");
        Arc::new(Mutex::new(conn))
    };
}

// operation table accounts
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
// end operation table accounts

// use std::time::{SystemTime, UNIX_EPOCH};

// fn main() {
//     let start = SystemTime::now();
//     let since_the_epoch = start.duration_since(UNIX_EPOCH)
//         .expect("Time went backwards");
//     println!("{:?}", since_the_epoch);
// }

// operation table articles
#[post("/article/add", format = "application/json", data = "<data>")]
fn tambah_article(data: Json<AddArticle>) -> Json<JsonValue> {
    let conn = DB.lock().unwrap();

    // use chrono::{NaiveDate, TimeZone};
    // use chrono_tz::Asia::Jakarta;

    // let naive_dt = NaiveDate::from_ymd(2038, 1, 19).and_hms(3, 14, 08);
    // let tz_aware = Jakarta.from_local_datetime(&naive_dt).unwrap();
    // assert_eq!(tz_aware.to_string(), "2038-01-19 03:14:08 SAST");

    let now = Local::now().naive_local();

    let new_article = models::NewArticle {
        judul: &data.judul,
        konten: &data.konten,
        waktu: now,
        penulis: &data.penulis,
    };

    let article: models::Article = diesel::insert_into(schema::articles::table)
        .values(&new_article)
        .get_result(&*conn)
        .expect("gagal insert akun ke dalam database");

    Json(
        serde_json::value::to_value(ApiResultArticle {
            result: vec![article],
        })
        .expect("gagal meng-serialize data"),
    )
}

#[get("/article")]
fn daftar_article() -> Json<ApiResultArticle> {
    let conn = DB.lock().unwrap();

    use schema::articles::dsl::*;

    let daftar = articles
        .limit(10)
        .load::<models::Article>(&*conn)
        .expect("gagal query ke database");

    Json(ApiResultArticle { result: daftar })
}

#[post("/article/update", data = "<data>")]
fn update_article(data: Json<UpdateArticle>) -> Json<ApiResultArticle> {
    let conn = DB.lock().unwrap();

    use schema::articles::dsl::*;

    let now = Local::now().naive_local();

    let data_baru: models::Article = diesel::update(articles.find(data.id))
        .set((
            judul.eq(&data.judul),
            konten.eq(&data.konten),
            waktu.eq(now),
            penulis.eq(&data.penulis),
        ))
        .get_result::<models::Article>(&*conn)
        .expect("gagal update ke database");

    Json(ApiResultArticle {
        result: vec![data_baru],
    })
}

#[post("/article/delete", data = "<data>")]
fn delete_article(data: Json<IdQueryArticle>) -> String {
    let conn = DB.lock().unwrap();

    use schema::articles::dsl::*;

    diesel::delete(articles.find(data.id))
        .execute(&*conn)
        .expect("gagal menghapus dari database");

    format!("Artikel dengan id `{}` telah dihapus.\n", data.id)
}
// end operation table articles

fn main() {
    dotenv().ok();
    // let start = SystemTime::now();
    // let since_the_epoch = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");
    // println!("{:?}", since_the_epoch);

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
                delete,
                daftar_article,
                tambah_article,
                delete_article,
                update_article
            ],
        )
        .launch();
}
