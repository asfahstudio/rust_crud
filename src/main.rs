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

mod schema;
mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use rocket_contrib::json::Json;
use serde_json::{Value as Json_Value};

use rocket::http::RawStr;
use rocket::response::NamedFile;
use std::path::{PathBuf, Path};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/nama/<name>/<umur>")]
fn nama_user(name: &RawStr, umur: i32) -> String {

    let pak_atau_mas = if umur <= 20 {
        "Dik"
    }else if umur <= 30 {
        "Mas"
    }else if umur <= 50{
        "Pak"
    }else{
        "Mbah"
    };

    let pesan = if umur <= 20 {
        "Anda Masih Kesil"
    }else if umur <= 30 {
        "Anda Masih Muda"
    }else if umur <= 50 {
        "Anda Sudah Tua"
    }else{
        "Tobat Boss !!!"
    };

    format!("Hello, {} {} {}!", pak_atau_mas, name.as_str(), pesan)
}

#[get("/download/<file..>")]
fn ke_file(file:PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[derive(Serialize, Deserialize)]
struct Anggota {
    pub nama: String,
    pub email: String,
    //pub umur: String,
    pub alamat: String
}

#[derive(Serialize, Deserialize)]
struct UpdateAnggota {
    pub id:i64,
    pub nama: String,
    pub email: String,
    //pub umur: String,
    pub alamat: String
}

#[derive(Serialize, Deserialize)]
struct IdQuery {
    pub id:i64
}

#[derive(Serialize)]
struct ApiResult {
    pub result: Vec<models::Account>,
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref DB:Arc<Mutex<PgConnection>> = {
        let database_url = env::var("DATABASE_URL")
            .expect("variable DATABASE_URL belum di Set");
        let conn = PgConnection::establish(&database_url)
            .expect("Gagal melakukan Koneksi ke Database");
        
        Arc::new(Mutex::new(conn))
    };
}

#[post("/register", format = "application/json", data = "<data>")]
fn register(data: Json<Anggota>) -> Json<Json_Value> {
    //Json(ApiResult{
    //    result: data.into_inner(),
    //})
    
    // if data.email.is_none(){
    //     return Json(json!({"error" : "email Kosong Bos !!!"}));
    // }

    // let mut db = DB.lock().unwrap();
    // db.insert(data.nama.to_owned(), data.email.as_ref().unwrap().to_owned());

    let conn = DB.lock().unwrap();

    let new_account = models::NewAccount {
        nama: &data.nama,
        email: &data.email,
        alamat: &data.alamat,
    };

    let account:models::Account = diesel::insert_into(schema::accounts::table)
        .values(&new_account)
        .get_result(&*conn)
        .expect("Gagal insert Akun ke dalam Database");


    // dbg!(&*db);
    
    Json(
        serde_json::value::to_value(
            ApiResult{
                result: vec![account],
            }
        ).expect("Serialize Gagal !!!"),
    )
}

#[get("/anggota")]
fn daftar_anggota() -> Json<ApiResult>{
    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;
    
    let daftar = accounts.limit(10).load::<models::Account>(&*conn)
        .expect("Gagal Query ke Database");



    // let daftar = db
    //     .iter()
    //     .map(|(k, v)| Anggota{
    //         nama : k.to_owned(),
    //         email : Some(v.to_owned()),
    //     })
    //     .collect();

    Json(ApiResult{result: daftar})
}

#[post("/anggota/update", data="<data>")]
fn update(data: Json<UpdateAnggota>) -> Json<ApiResult> {
    let conn = DB.lock().unwrap();

    // let mut data_baru : Option<Anggota> = None;
    
    // match db.iter_mut().find(|(k, _v)| *k == &data.nama) {
    //     Some((k, v)) => {
    //         *v = data.email.as_ref().unwrap().to_owned();
    //         data_baru = Some(Anggota{
    //             nama: k.to_owned(),
    //             email: Some(v.to_owned()),
    //         });
    //     }
    //     None => (),
    // }

    use schema::accounts::dsl::*;

    let data_baru:models::Account = diesel::update(accounts.find(data.id))

        .set((nama.eq(&data.nama), email.eq(&data.email), alamat.eq(&data.alamat)))
        .get_result::<models::Account>(&*conn)
        .expect("Gagal Update ke Database");

    Json(ApiResult{
        result: vec![data_baru],
    })
}

#[post("/anggota/delete", data="<data>")]
fn delete(data: Json<IdQuery>) -> String {
    // let mut db = DB.lock().unwrap();

    // db.remove(&data.nama);

    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;

    diesel::delete(accounts.find(data.id))
        .execute(&*conn)
        .expect("Gagal Menghapus dari Dtabase");

    format!("Akun Anggota dengan id '{}' Telah di Hapus.", data.id)
}

fn main() {
    dotenv().ok();
    
    rocket::ignite()
        .mount(
            "/",
            routes![index, nama_user, ke_file, register,
            daftar_anggota, update, delete],
            )
            .launch();
}