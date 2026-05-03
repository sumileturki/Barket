use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use diesel::associations::HasTable;
use diesel::prelude::*;
use crate::models::product::{NewProduct, Product};
use crate::schema::products::dsl::*;
use crate::DbPool;
use crate::utils::jwt::Claims; 



#[derive(AsChangeset, serde::Deserialize)]
#[diesel(table_name = crate::schema::products)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<bigdecimal::BigDecimal>,
    pub stock: Option<i32>,
}



#[post("/create_product")]
pub async fn create_product(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    item: web::Json<NewProduct>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection");

    let extensions = req.extensions();


    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    let mut new_product = item.into_inner();

    new_product.seller_id = Some(claims.sub); // assuming sub = user_id

    let result = diesel::insert_into(products)
        .values(&new_product)
        .get_result::<Product>(&mut conn);

    match result {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(err) => {
            println!("Error: {:?}", err);
            HttpResponse::InternalServerError().body("Error creating product")
        }
    }
}

#[get("/products")]
pub async fn get_allproduct(
    pool : web::Data<DbPool>
)-> impl Responder{
    let mut conn = pool.get().expect("couldn't get db connection");

    // let extensions = req.extensions();
    
    // let claims = match extensions.get::<Claims>() {
    //     Some(c) => c,
    //     None => return HttpResponse::Unauthorized().body("Unauthorized"),
    // };

     let result = products
        .filter(is_active.eq(true))
        .load::<Product>(&mut conn);

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/products/{id}")]
pub async fn get_product(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let product_id = path.into_inner();

    let result = products
        .filter(id.eq(product_id))
        .first::<Product>(&mut conn);

    match result {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::NotFound().body("Product not found"),
    }
}



#[put("/products/{id}")]
pub async fn update_product(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    item: web::Json<UpdateProduct>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let product_id = path.into_inner();

    // 🔒 check ownership
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let existing = products
        .filter(id.eq(product_id))
        .first::<Product>(&mut conn);

    let existing = match existing {
        Ok(p) => p,
        Err(_) => return HttpResponse::NotFound().body("Product not found"),
    };

    if existing.seller_id != Some(claims.sub) {
        return HttpResponse::Forbidden().body("Not your product");
    }

    let result = diesel::update(products.filter(id.eq(product_id)))
        .set(&item.into_inner())
        .get_result::<Product>(&mut conn);

    match result {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/products/{id}")]
pub async fn delete_product(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let product_id = path.into_inner();

    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let existing = products
        .filter(id.eq(product_id))
        .first::<Product>(&mut conn);

    let existing = match existing {
        Ok(p) => p,
        Err(_) => return HttpResponse::NotFound().body("Product not found"),
    };

    if existing.seller_id != Some(claims.sub) {
        return HttpResponse::Forbidden().body("Not your product");
    }

    let result = diesel::update(products.filter(id.eq(product_id)))
        .set(is_active.eq(false))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().body("Product deleted"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}