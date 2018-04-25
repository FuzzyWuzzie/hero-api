extern crate jsonwebtoken as jwt;

//use self::jwt::{encode, decode, Header, Algorithm, Validation};
use self::jwt::{encode, Header};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss:String,
    uid:u32
}

pub fn build_token(uid:u32)->String {
    let claims = Claims {
        iss: "hero-api".to_owned(),
        uid
    };

    let token = encode(&Header::default(), &claims, "my super secret".as_ref()).unwrap();
    token
}

//pub fn validate_token(token:&str)->bool {
//    false
//}