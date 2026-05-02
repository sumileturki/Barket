use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, HttpMessage,
};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::task::{Context, Poll};

use crate::utils::jwt::{verify_token, Claims};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareInner { service })
    }
}

pub struct AuthMiddlewareInner<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let path = req.path();

        // ✅ PUBLIC ROUTES ONLY (NO METHOD BASED BYPASS)
        let public_routes = ["/api/login", "/api/register"];

        if public_routes.contains(&path) {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }

        let token = req
            .headers()
            .get("Authorization")
            .and_then(|hv| hv.to_str().ok())
            .and_then(|auth| auth.strip_prefix("Bearer "));

        match token {
            Some(token) => {
                match verify_token(token) {
                    Some(claims) => {
                        req.extensions_mut().insert::<Claims>(claims);

                        let fut = self.service.call(req);
                        Box::pin(async move { fut.await })
                    }
                    None => {
                        println!("❌ Invalid or expired token");
                        Box::pin(async {
                            Err(actix_web::error::InternalError::from_response(
                                "",
                                HttpResponse::Unauthorized()
                                    .json("Invalid or expired token"),
                            )
                            .into())
                        })
                    }
                }
            }
            None => {
                println!("❌ Missing Authorization header");
                Box::pin(async {
                    Err(actix_web::error::InternalError::from_response(
                        "",
                        HttpResponse::Unauthorized()
                            .json("Authorization header missing"),
                    )
                    .into())
                })
            }
        }
    }
}