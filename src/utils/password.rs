use bcrypt::{DEFAULT_COST, verify};


pub fn hashpassword(pw:&str)-> String {
    bcrypt::hash(pw, DEFAULT_COST).expect("msg")
}

pub fn verify_password(pw: &str, hash: &str) -> bool {
    verify(pw, hash).unwrap_or(false)
}