#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rusqlite;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::State;
use rocket_contrib::{Json, Value};

mod hero;
use hero::Hero;

mod db;
use std::sync::Mutex;

mod auth;
use auth::{AuthBasicSuccess, AuthToken, IsAdmin};

mod tokens;

#[get("/")]
fn signin(auth:AuthBasicSuccess)->Json<Value> {
    Json(json!({
        "token": tokens::build_token(auth.uid, auth.adm)
    }))
}

#[derive(Serialize, Deserialize, Debug)]
struct Credentials {
    name:String,
    pass:String,
    admin:bool
}

#[post("/", data="<credentials>")]
fn create_user(conn: State<db::DBConn>, credentials: Json<Credentials>, _auth: AuthToken, _adm: IsAdmin) -> Json<Value> {
    let conn = conn.lock()
        .expect("db connection lock");
    let uid = auth::register_user(&conn, &credentials.name, &credentials.pass, &credentials.admin).unwrap();
    Json(json!({
        "uid": uid
    }))
}

#[post("/", data="<hero>")]
fn create(conn: State<db::DBConn>, hero:Json<Hero>, _auth: AuthToken)->Json<Hero> {
    let conn = conn.lock()
        .expect("db connection lock");
    Json(Hero::create(&conn, Hero { id: None, ..hero.into_inner() }))
}

#[get("/")]
fn read(conn: State<db::DBConn>)->Json<Value> {
    let conn = conn.lock()
        .expect("db connection lock");
    Json(json!(Hero::read(&conn)))
}

#[get("/<id>")]
fn read_single(conn: State<db::DBConn>, id:i32) -> Json<Hero> {
    let conn = conn.lock()
        .expect("db connection lock");
    Json(Hero::read_single(&conn, id))
}

#[put("/<id>", data="<hero>")]
fn update(conn: State<db::DBConn>, id:i32, hero:Json<Hero>, _auth: AuthToken)->Json<Hero> {
    let conn = conn.lock()
        .expect("db connection lock");
    Json(Hero::update(&conn, id, &hero))
}

#[delete("/<id>")]
fn delete(conn: State<db::DBConn>, id:i32, _auth: AuthToken)->Json<Value> {
    let conn = conn.lock()
        .expect("db connection lock");
    let status:bool = Hero::delete(&conn, id);
    Json(json!({
        "deleted": status
    }))
}

fn main() {
    let conn = db::get_connection("heroes.db")
        .expect("open database");
    match db::initialize(&conn) {
        Ok(_) => (),
        Err(e) => {
            println!("error initializing:\n{}", e);
        }
    };
    

    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/auth", routes![signin, create_user])
        .mount("/hero", routes![create, read_single, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}