use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

const SECRET: &[u8] = b"secret_key";

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

pub fn create_access_token(user_id: i32) -> String {
    let exp = (Utc::now() + Duration::minutes(15)).timestamp() as usize;

    let claims = Claims { sub: user_id, exp };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

pub fn create_refresh_token(user_id: i32) -> (String, chrono::NaiveDateTime) {
    let exp_time = Utc::now() + Duration::days(7);

    let claims = Claims {
        sub: user_id,
        exp: exp_time.timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap();

    (token, exp_time.naive_utc()) 
}

pub fn verify_token(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .map(|d| d.claims)
    .ok()
}