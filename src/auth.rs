extern crate rocket;
extern crate base64;
extern crate bcrypt;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::State;

use self::bcrypt::{DEFAULT_COST, hash, verify};

use rusqlite::Connection;
use tokens;
use db::DBConn;

#[derive(Debug)]
pub struct AuthBasicSuccess {
    pub uid: u32,
    pub adm:bool
}

#[derive(Debug)]
pub struct AuthToken {
    pub uid:u32,
    pub adm:bool
}

pub struct IsAdmin();

impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthToken, ()> {
        let auths:Vec<_> = request.headers().get("Authorization").collect();
        if auths.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        let auth:Vec<&str> = auths[0].split(' ').collect();
        if auth.len() != 2 || auth[0] != "Bearer" {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        let token = auth[1];

        let config = request.guard::<State<super::Config>>()?;
        let token = match tokens::validate_token(&config.secret, &token) {
            Ok(tok) => tok,
            Err(_) => {
                println!("Invalid token!");
                return Outcome::Failure((Status::Unauthorized, ()))
            }
        };

        Outcome::Success(token)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for IsAdmin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<IsAdmin, ()> {
        let token = request.guard::<AuthToken>()?;
        if !token.adm {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        Outcome::Success(IsAdmin())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthBasicSuccess {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthBasicSuccess, ()> {
        let auths:Vec<_> = request.headers().get("Authorization").collect();
        if auths.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let auth:Vec<&str> = auths[0].split(' ').collect();
        if auth.len() != 2 || auth[0] != "Basic" {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        
        let auth = auth[1];
        let auth = base64::decode(&auth).unwrap();
        let auth:String = String::from_utf8(auth).unwrap();

        let auth:Vec<&str> = auth.split(':').collect();
        if auth.len() < 2 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        let user:&str = auth[0];
        let pass:&str = &auth[1..].join(":");

        let conn = request.guard::<State<DBConn>>()?;
        let conn = conn.lock()
            .expect("db connection lock");
        let mut stmt = conn.prepare("select id, pass, admin from users where name=?1").unwrap();
        let (uid, hash, adm):(u32, String, bool) = match stmt.query_row(&[&user], |row| {
            let admin:i32 = row.get(2);
            (row.get(0), row.get(1), admin == 1)
        }) {
            Ok(derp) => derp,
            Err(_) => return Outcome::Failure((Status::Unauthorized, ()))
        };

        if verify(pass, &hash).is_err() {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        return Outcome::Success(AuthBasicSuccess{
            uid,
            adm
        });
    }
}

pub fn register_user(conn:&Connection, name:&str, pass:&str, admin:&bool)->Result<i32, ()> {
    let hashed_pass = match hash(pass, DEFAULT_COST) {
        Ok(v) => v,
        Err(_) => return Err(())
    };

    let adm:i32 = if *admin { 1 } else { 0 };
    let result = conn.execute(
        "insert into users(name, pass, admin) values(?1, ?2, ?3)",
        &[&name, &hashed_pass, &adm]
    );
    if result.is_err() {
        return Err(());
    }

    let mut stmt = conn.prepare("select id from users order by id desc limit 1").unwrap();
    let id:i32 = match stmt.query_row(&[], |row| {
        row.get(0)
    }) {
        Ok(id) => id,
        Err(_) => return Err(())
    };

    Ok(id)
}