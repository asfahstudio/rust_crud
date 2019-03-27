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
extern crate rand;

// #[macro_use]
extern crate chrono;
extern crate chrono_tz;

mod models;
mod schema;

use chrono::prelude::Local;
use serde::Serialize;

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

// accounts serde start
#[derive(Serialize, Deserialize)]
struct Anggota {
    pub nama: String,
    pub email: String,
    pub alamat: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateAnggota {
    pub id: i64,
    pub nama: String,
    pub email: String,
    pub alamat: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct IdQuery {
    pub id: i64,
}

#[derive(Serialize)]
struct ApiResult<T: Serialize> {
    pub result: T,
}
// accounts serde end

// articles serde start
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
// articles serde end

use std::sync::{Arc, Mutex};

lazy_static! {
    static ref DB: Arc<Mutex<PgConnection>> = {
        let database_url = env::var("DATABASE_URL").expect("variable DATABASE_URL belum diset!");
        let conn =
            PgConnection::establish(&database_url).expect("gagal melakukan koneksi ke database");
        Arc::new(Mutex::new(conn))
    };
}

// accounts CRUD start
#[post("/register", format = "application/json", data = "<data>")]
fn register(data: Json<Anggota>) -> Json<JsonValue> {
    let conn = DB.lock().unwrap();

    let new_account = models::NewAccount {
        nama: &data.nama,
        email: &data.email,
        alamat: &data.alamat,
        password: &data.password,
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

#[derive(Serialize, Deserialize)]
struct Login {
    pub email: String,
    pub password: String,
}

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn generate_token_code() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(80).collect()
}

#[post("/authorize", data = "<data>")]
fn authorize(data: Json<Login>) -> Json<ApiResult<String>> {
    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;

    let account: models::Account = accounts
        .filter(email.eq(&data.email))
        .first(&*conn)
        .expect("gagal mendapatkan user");

    // cocokan password
    if data.password != account.password {
        return Json(ApiResult {
            result: String::new(),
        });
    }

    let access_token = generate_token_code();

    {
        let at = models::NewAccessToken {
            user_id: account.id,
            token: &access_token,
        };

        diesel::insert_into(schema::access_tokens::table)
            .values(&at)
            .execute(&*conn)
            .expect("gagal menambahkan entry access token di db");
    }

    Json(ApiResult {
        result: access_token,
    })
}

#[get("/anggota")]
fn daftar_anggota() -> Json<ApiResult<Vec<models::Account>>> {
    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;

    let daftar = accounts
        .limit(10)
        .load::<models::Account>(&*conn)
        .expect("gagal query ke database");

    Json(ApiResult { result: daftar })
}

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

struct AuthOnly {
    pub access_token: String,
    pub account: models::Account,
}

fn account_from_token(access_token: &str) -> Result<models::Account, &'static str> {
    use schema::access_tokens::dsl as dslt;
    use schema::accounts::dsl as dsla;

    let conn = DB.lock().unwrap();

    let at: models::AccessToken = dslt::access_tokens
        .filter(dslt::token.eq(access_token))
        .first(&*conn)
        .map_err(|_| "no access token in db")?;

    let account: models::Account = dsla::accounts
        .find(at.user_id)
        .first(&*conn)
        .map_err(|_| "account not exists")?;

    Ok(account)
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthOnly {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<&str> = request.headers().get("X-Access-Token").collect();
        dbg!(&keys);
        match keys.len() {
            1 => {
                let access_token = keys[0];

                let account = account_from_token(access_token)
                    .map_err(|e| Err((Status::Unauthorized, format!("{}", e))))?;

                let access_token = access_token.to_string();

                Outcome::Success(AuthOnly {
                    access_token,
                    account,
                })
            }
            _ => Outcome::Failure((Status::Unauthorized, "No X-Access-Token header".to_string())),
        }
    }
}

#[post("/anggota/update", data = "<data>")]
fn update(data: Json<UpdateAnggota>, auth: AuthOnly) -> Json<ApiResult<Vec<models::Account>>> {
    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;

    println!(
        "user yang mengakses /anggota/update: '{}'",
        auth.account.nama
    );

    let data_baru: models::Account = diesel::update(accounts.find(data.id))
        .set((
            nama.eq(&data.nama),
            email.eq(&data.email),
            alamat.eq(&data.alamat),
            password.eq(&data.password),
        ))
        .get_result::<models::Account>(&*conn)
        .expect("gagal update ke database");

    Json(ApiResult {
        result: vec![data_baru],
    })
}

#[post("/anggota/delete", data = "<data>")]
fn delete(data: Json<IdQuery>, auth: AuthOnly) -> String {
    let conn = DB.lock().unwrap();

    use schema::accounts::dsl::*;
    println!(
        "user yang mengakses /anggota/delete: '{}'",
        auth.account.nama
    );

    diesel::delete(accounts.find(data.id))
        .execute(&*conn)
        .expect("gagal menghapus dari database");

    format!("Akun anggota id `{}` telah dihapus.\n", data.id)
}
// accounts CRUD end

// articles CRUD start
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
// articles CRUD end

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
                update_article,
                authorize
            ],
        )
        .launch();
}
