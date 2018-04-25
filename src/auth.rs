extern crate rocket;
extern crate base64;
extern crate bcrypt;


use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::State;
use self::bcrypt::{DEFAULT_COST, hash, verify};
use db::DBConn;

pub struct AuthToken {
    pub uid:u32
}

fn is_valid(key: &str)->bool {
    println!("validating key: {}", key);
    key == "Bearer derp"
}

impl AuthToken {
    pub fn from_auth(_auth: &str)->AuthToken {
        AuthToken {
            uid: 1 // TODO: actually parse the token!
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthToken, ()> {
        let auths:Vec<_> = request.headers().get("Authorization").collect();
        if auths.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let auth = auths[0];
        if !is_valid(auth) {
            //return Outcome::Forward(());
            println!("not valid auth!");
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        return Outcome::Success(AuthToken::from_auth(auth));
    }
}

#[derive(Debug)]
pub struct AuthBasicSuccess {
    pub uid: u32
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
        let mut stmt = conn.prepare("select id, pass from users where name=?1").unwrap();
        let (uid, hash):(u32, String) = stmt.query_row(&[&user], |row| {
            (row.get(0), row.get(1))
        }).unwrap();

        if verify(pass, &hash).is_err() {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        return Outcome::Success(AuthBasicSuccess{
            uid
        });
    }
}