extern crate rusqlite;

use self::rusqlite::{Connection, Error};
use std::sync::Mutex;

pub type DBConn = Mutex<Connection>;

pub fn initialize(conn: &Connection) -> Result<(), Error> {
    conn.execute("CREATE TABLE heroes (
        id integer primary key,
        name varchar(255) not null,
        identity varchar(255) not null,
        hometown varchar(255) not null,
        age int not null
    )", &[])?;
    conn.execute("CREATE TABLE users (
        id integer priamry key,
        name varchar(255) not null,
        pass varchar(255) not null
    )", &[])?;
    println!("Initialized table");
    Ok(())
}

pub fn get_connection(location: &str) -> Result<Connection, Error> {
    Connection::open(location)
}
