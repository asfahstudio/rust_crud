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
extern crate chrono;
extern crate dotenv;

mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use chrono::prelude::Local;

use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use serde_json::Value as JsonValue;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/nama/<name>/<umur>")]
fn nama_user(name: &RawStr, umur: i32) -> String {
    //   let umur:i32 = match umur.parse(){
    //    Ok(angka) => angka,
    //    Err(gagal) =>{
    //       println!("Umur bukan angka: {}",umur)
    //     20
    //    }
    //  };
    let pak_atau_mas = if umur > 30 { "pak" } else { "mas" };
    format!(
        "Hello, {} {}! \nApa kabarmu di umur ({})?",
        pak_atau_mas,
        name.as_str(),
        umur
    )
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
    pub waktu: String,
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

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    // static  ref DB:Arc<Mutex<HashMap<String, String>>> = {
    //     let d = HashMap::new();
    //     //d.insert("yoga".to_string(), r#"{"nama":"email":"yoga@gmail.com"}"#to_string());
    //     Arc::new(Mutex::new(d))
    // };

    static ref DB:Arc<Mutex<PgConnection>> = {
        let database_url = env::var("DATABASE_URL").expect("variable DATABASE_URL belum diset");
        let conn = PgConnection::establish(&database_url).expect("Gagal melakukan  koneksi ke database");
        Arc::new(Mutex::new(conn))
    };
}

#[post("/register", format = "application/json", data = "<data>")]
fn register(data: Json<Anggota>) -> Json<JsonValue> {
    // Json(ApiResult {
    //     result: data.into_inner(),
    // })

    // if data.email.is_none(){
    //     return Json(json!({"error":"email kosong, harus diisi!"}));
    // }
    // let mut db = DB.lock().unwrap();
    // db.insert(data.nama.to_owned(), data.email.as_ref().unwrap().to_owned());
    // dbg! (&*db);

    let conn = DB.lock().unwrap();

    let new_account = models::NewAccount {
        nama: &data.nama,
        email: &data.email,
        alamat: &data.alamat,
    };

    let account: models::Account = diesel::insert_into(schema::accounts::table)
        .values(&new_account)
        .get_result(&*conn)
        .expect("Gagal insert akun ke dalam database");

    Json(
        serde_json::value::to_value(ApiResult {
            result: vec![account],
        })
        .expect("Gagal meng-serialize data"),
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

    // let daftar = db
    // .iter()
    // .map(|(k,v)| Anggota{
    //     nama: k.to_owned(),
    //     email: Some(v.to_owned()),
    // })
    // .collect();

    Json(ApiResult { result: daftar })
}

#[post("/anggota/update", data = "<data>")]
fn update(data: Json<UpdateAnggota>) -> Json<ApiResult> {
    let conn = DB.lock().unwrap();
    // let mut data_baru: Option<Anggota> = None;

    // match db.iter_mut().find(|(k,_v)| *k == &data.nama) {
    //     Some((k, v)) => {
    //         *v = data.email.as_ref().unwrap().to_owned();
    //         data_baru = Some(Anggota {
    //             nama: k.to_owned(),
    //             email: Some(v.to_owned())
    //         });
    //     }
    //     None => (),
    // }

    use schema::accounts::dsl::*;

    let data_baru: models::Account = diesel::update(accounts.find(data.id))
        //nama = data.nama, email = data.email, alamat = data.alamat
        .set((
            nama.eq(&data.nama),
            email.eq(&data.email),
            alamat.eq(&data.alamat),
        ))
        .get_result::<models::Account>(&*conn)
        .expect("Gagal mengupdate ke database");

    Json(ApiResult {
        result: vec![data_baru],
    })
}

#[post("/anggota/delete", data = "<data>")]
fn delete(data: Json<IdQuery>) -> String {
    // let mut db = DB.lock().unwrap();
    // match db.remove.find(|(k,_v)| *k == &data.nama)
    // db.remove(&data.nama);

    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;

    diesel::delete(accounts.find(data.id))
        .execute(&*conn)
        .expect("Gagal menghapus dari database");

    format!("Akun anggota '{}' telah dihapus.", data.id)
}

#[post("/article/add", format = "application/json", data = "<data>")]
fn tambah_article(data: Json<AddArticle>) -> Json<JsonValue> {
    let conn = DB.lock().unwrap();

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

    let _update_now = Local::now().naive_local();

    use schema::articles::dsl::*;

    let data_baru: models::Article = diesel::update(articles.find(data.id))
        .set((
            judul.eq(&data.judul),
            konten.eq(&data.konten),
            waktu.eq(_update_now),
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

    format!("Akun anggota dengan id `{}` telah dihapus.\n", data.id)
}
// end operation table articles

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
                delete,
                daftar_article,
                tambah_article,
                delete_article,
                update_article
            ],
        )
        .launch();
}