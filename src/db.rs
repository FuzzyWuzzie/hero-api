use rusqlite::{Connection, Error};
use std::sync::Mutex;

use auth;

pub type DBConn = Mutex<Connection>;

pub fn initialize(conn: &Connection) -> Result<(), Error> {
    match conn.execute("CREATE TABLE heroes (
        id integer primary key,
        name varchar(255) not null,
        identity varchar(255) not null,
        hometown varchar(255) not null,
        age int not null
    )", &[]) {
        Ok(_) => println!("Created heroes table!"),
        Err(e) => println!("Didn't create heroes table: {:?}", match e {
            Error::SqliteFailure(_, desc) => match desc {
                Some(deets) => deets,
                None => "?".to_string()
            },
            _ => format!("{:?}", e)
        })
    };

    match conn.execute("CREATE TABLE users (
        id integer primary key,
        name varchar(255) not null,
        pass varchar(60) not null,
        admin integer not null
    )", &[]) {
        Ok(_) => println!("Created users table!"),
        Err(e) => println!("Didn't create users table: {:?}", match e {
            Error::SqliteFailure(_, desc) => match desc {
                Some(deets) => deets,
                None => "?".to_string()
            },
            _ => format!("{:?}", e)
        })
    };

    let mut stmt = conn.prepare("select count(id) from users where admin=1").unwrap();
    let count:i32 = stmt.query_row(&[], |row| {
        row.get(0)
    }).expect("querying admin count");

    if count < 1 {
        println!("Creating admin user...");
        match auth::register_user(&conn, "admin", "password", &true) {
            Ok(id) => println!("Created admin with id {}", id),
            Err(_) => println!("Failed to create admin user!")
        };
    }

    println!("Initialized tables!");
    Ok(())
}

pub fn get_connection(location: &str) -> Result<Connection, Error> {
    Connection::open(location)
}
