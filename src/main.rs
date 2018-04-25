#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::State;
use rocket_contrib::{Json, Value};

mod hero;
use hero::Hero;

mod db;
use std::sync::Mutex;

#[post("/", data="<hero>")]
fn create(conn: State<db::DBConn>, hero:Json<Hero>)->Json<Hero> {
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
fn update(conn: State<db::DBConn>, id:i32, hero:Json<Hero>)->Json<Hero> {
    let conn = conn.lock()
        .expect("db connection lock");
    Json(Hero::update(&conn, id, &hero))
}

#[delete("/<id>")]
fn delete(conn: State<db::DBConn>, id:i32)->Json<Value> {
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
        .mount("/hero", routes![create, read_single, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}