#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;
extern crate rocket_contrib;

// #[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate chrono_tz;

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

// use chrono::{TimeZone, Utc, NaiveDateTime};
// use chrono_tz::Asia::Jakarta;
// use chrono::prelude::*;
// use std::time::SystemTime;
// use diesel::dsl::now;

use rocket_contrib::json::Json;
use serde_json::Value as JsonValue;

use rocket::http::RawStr;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};


/*------------------------------------------------------------------------------------- */

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/nama/<name>/<umur>")]
fn nama_user(name: &RawStr, umur: i32) -> String {
    let pak_atau_mas = if umur <= 20 {
        "Dik"
    } else if umur <= 30 {
        "Mas"
    } else if umur <= 50 {
        "Pak"
    } else {
        "Mbah"
    };

    let pesan = if umur <= 20 {
        "Anda Masih Kesil"
    } else if umur <= 30 {
        "Anda Masih Muda"
    } else if umur <= 50 {
        "Anda Sudah Tua"
    } else {
        "Tobat Boss !!!"
    };

    format!("Hello, {} {} {}!", pak_atau_mas, name.as_str(), pesan)
}

#[get("/download/<file..>")]
fn ke_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/*------------------------------------------------------------------------------------- */
/* ANGGOTA */

#[derive(Serialize, Deserialize)]
struct Anggota {
    pub nama: String,
    pub email: String,
    //pub umur: String,
    pub alamat: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateAnggota {
    pub id: i64,
    pub nama: String,
    pub email: String,
    //pub umur: String,
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

/* END of ANGGOTA */
/*------------------------------------------------------------------------------------- */
/* ARTIKEL */

// #[derive(Serialize, Deserialize)]
// struct AddArtikel {
//     pub judul: String,
//     pub konten: String,
//     pub writer: String,
// }

// #[derive(Serialize, Deserialize)]
// struct UpdateArtikel {
//     pub id_artikels: i64,
//     pub judul: String,
//     pub konten: String,
//     pub writer: String,
// }

// #[derive(Serialize, Deserialize)]
// struct IdQueryArtikel {
//     pub id_artikels: i64,
// }

// #[derive(Serialize)]
// struct ApiResultArtikel {
//     pub result: Vec<models::Artikel>,
// }

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


/*END of ARTIKEL */
/* -------------------------------------------------------------------------------------------------- */

// use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref DB: Arc<Mutex<PgConnection>> = {
        let database_url = env::var("DATABASE_URL").expect("variable DATABASE_URL belum di Set");
        let conn =
            PgConnection::establish(&database_url).expect("Gagal melakukan Koneksi ke Database");

        Arc::new(Mutex::new(conn))
    };
}

/* -------------------------------------------------------------------------------------------------- */
/* ANGGOTA */

#[post("/register", format = "application/json", data = "<data>")]
fn register(data: Json<Anggota>) -> Json<JsonValue> {
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

    let account: models::Account = diesel::insert_into(schema::accounts::table)
        .values(&new_account)
        .get_result(&*conn)
        .expect("Gagal insert Akun ke dalam Database");

    // dbg!(&*db);

    Json(
        serde_json::value::to_value(ApiResult {
            result: vec![account],
        })
        .expect("Serialize Gagal !!!"),
    )
}

#[get("/anggota")]
fn daftar_anggota() -> Json<ApiResult> {
    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;

    let daftar = accounts
        .limit(10)
        .load::<models::Account>(&*conn)
        .expect("Gagal Query ke Database");

    // let daftar = db
    //     .iter()
    //     .map(|(k, v)| Anggota{
    //         nama : k.to_owned(),
    //         email : Some(v.to_owned()),
    //     })
    //     .collect();

    Json(ApiResult { result: daftar })
}

#[post("/anggota/update", data = "<data>")]
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

    let data_baru: models::Account = diesel::update(accounts.find(data.id))
        .set((
            nama.eq(&data.nama),
            email.eq(&data.email),
            alamat.eq(&data.alamat),
        ))
        .get_result::<models::Account>(&*conn)
        .expect("Gagal Update ke Database");

    Json(ApiResult {
        result: vec![data_baru],
    })
}

#[post("/anggota/delete", data = "<data>")]
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

/* END of ANGGOTA */
/* ---------------------------------------------------------------------------------------------- */
/* TABLE ARTIKEL */

// #[post("/artikel/add", format = "application/json", data = "<data>")]
// fn tulis_artikel(data: Json<AddArtikel>) -> Json<JsonValue> {
//     let conn = DB.lock().unwrap();

//     let new_artikel = models::NewArtikel {
//         judul: &data.judul,
//         konten: &data.konten,
//         writer: &data.writer,
//     };

//     // let dt = Jakarta.ymd(2016, 5, 10).and_hms(12, 0, 0);
//     // assert_eq!(dt.to_string(), "2016-05-10T12:00:00 WIB");
//     // assert_eq!(dt.to_rfc3339(), "2016-05-10T12:00:00+07:00");

//     // let SystemTime

//     let artikel: models::Artikel = diesel::insert_into(schema::artikels::table)
//         .values(&new_artikel)
//         .get_result::<models::Artikel>(&*conn)
//         .expect("Gagal insert Akun ke dalam Database");

//     // use chrono::TimeZone;
//     // use chrono_tz::Asia::Jakarta;
//     // let dt = Jakarta.ymd(2016, 5, 10).and_hms(12, 0, 0);
//     // assert_eq!(dt.to_string(), "2016-05-10T12:00:00 WIB");
//     // assert_eq!(dt.to_rfc3339(), "2016-05-10T12:00:00+07:00");

//     Json(
//         serde_json::value::to_value(ApiResultArtikel {
//             result: vec![artikel],
//         })
//         .expect("Serialize Gagal !!!"),
//     )
// }

// #[get("/artikel")]
// fn list_artikel() -> Json<ApiResultArtikel> {
//     let conn = DB.lock().unwrap();

//     use schema::artikels::dsl::*;

//     let list = artikels
//         .limit(10)
//         .load::<models::Artikel>(&*conn)
//         .expect("GAGAL QUERY KE DATABASE (ARTIKEL) !!!");

//     Json(ApiResultArtikel { result: list })
// }

// #[post("/artikel/update", data = "<data>")]
// fn update_artikel(data: Json<UpdateArtikel>) -> Json<ApiResultArtikel> {
//     let conn = DB.lock().unwrap();

//     use schema::artikels::dsl::*;

//     let artikel_baru: models::Artikel = diesel::update(artikels.find(data.id_artikels))
//         .set((
//             judul.eq(&data.judul),
//             konten.eq(&data.konten),
//             writer.eq(&data.writer),
//         ))
//         .get_result::<models::Artikel>(&*conn)
//         .expect("Gagal Update ke Database..!!");

//     Json(ApiResultArtikel {
//         result: vec![artikel_baru],
//     })
// }

// #[post("/artikel/delete", data = "<data>")]
// fn delete_artikel(data: Json<IdQueryArtikel>) -> String {
//     let conn = DB.lock().unwrap();

//     use schema::artikels::dsl::*;

//     diesel::delete(artikels.find(data.id_artikels))
//         .execute(&*conn)
//         .expect("GAgal Menghapus Artikel..!!");
//     format!(
//         "Tulisan Artikel dengan id '{}' TElah di Hapus !!",
//         data.id_artikels
//     )
// }

/* END of ARTIKEL */
/*------------------------------------------------------------------------------------- */
/* Kode FAJAR */

#[post("/article/add", format = "application/json", data = "<data>")]
fn tambah_article(data: Json<AddArticle>) -> Json<JsonValue> {
    let conn = DB.lock().unwrap();

    let new_article = models::NewArticle {
        judul: &data.judul,
        konten: &data.konten,
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

    let data_baru: models::Article = diesel::update(articles.find(data.id))
        .set((
            judul.eq(&data.judul),
            konten.eq(&data.konten),
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

    format!("Akun anggota id `{}` telah dihapus.\n", data.id)
}

/* END of Kode FAJAR */
/*------------------------------------------------------------------------------------- */

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
                tambah_article,
                daftar_article,
                update_article,
                delete_article
            ],
        )
        .launch();
}
