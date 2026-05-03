use actix_web::{HttpResponse, Responder, post, web};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    db::DbPool,
    models::user::{NewUser, User},
    utils::{
        jwt::{create_access_token, create_refresh_token},
        password::{hashpassword, verify_password},
    },
};

use crate::schema::refresh_token::dsl::*;
use crate::schema::users::dsl::*;

#[derive(Serialize)]
pub struct UserResponse {
    id: i32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct LoginInput {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LogoutInput {
    pub refresh_token: String,
}


#[post("/register")]
pub async fn registeruser(pool: web::Data<DbPool>, data: web::Json<NewUser>) -> impl Responder {
    let mut conn = pool.get().expect("DB connection failed");

    let newuser = NewUser {
        name: data.name.clone(),
        email: data.email.clone(),
        password: hashpassword(&data.password),
    };

    let inserted_user = diesel::insert_into(users)
        .values(&newuser)
        .get_result::<User>(&mut conn);

    match inserted_user {
        Ok(user) => HttpResponse::Ok().json(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        }),

        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => HttpResponse::BadRequest().body("Email already exists"),

        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

#[post("/login")]
pub async fn login(pool: web::Data<DbPool>, data: web::Json<LoginInput>) -> impl Responder {
    let mut conn = pool.get().unwrap();

    let user = users.filter(email.eq(&data.email)).first::<User>(&mut conn);

    if let Ok(user) = user {
        if verify_password(&data.password, &user.password) {
            let access_token = create_access_token(user.id);

            let (refresh_token_val, expiry_time) = create_refresh_token(user.id);
            // let expiry_time = expires_at_utc.naive_utc();

            diesel::insert_into(refresh_token)
                .values((
                    user_id.eq(Some(user.id)), 
                    token.eq(&refresh_token_val),
                    expires_at.eq(expiry_time),
                ))
                .execute(&mut conn)
                .ok();

            return HttpResponse::Ok().json(serde_json::json!({
                "access_token": access_token,
                "refresh_token": refresh_token_val
            }));
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}

#[post("/logout")]
pub async fn logout(
    pool: web::Data<DbPool>,
    data: web::Json<LogoutInput>,
)-> impl Responder{

    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB error"),
    };

    let token_val = &data.refresh_token;

    let result = diesel::delete(
        refresh_token.filter(token.eq(token_val))
    ).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Logged out successfully"
        })),
        Err(_) => HttpResponse::InternalServerError().body("Logout failed"),
    }
}

