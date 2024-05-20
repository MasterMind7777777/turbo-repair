use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, HttpMessage, HttpResponse};
use actix_web::body::BoxBody;
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::task::{Context, Poll};
use crate::utils::jwt::verify_jwt;
use crate::middleware::public::Public;
use log::info;

pub struct Auth;

impl<S> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Add logging to verify the request is being processed
        info!("Processing request: {:?}", req.path());

        // Check if the request has the Public attribute in extensions
        if req.extensions().get::<Public>().is_some() {
            info!("Public route, bypassing authentication");
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_boxed_body())
            });
        }

        // Check for Authorization header
        let headers = req.headers();
        let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = auth_header.trim_start_matches("Bearer ");
                if let Ok(_claims) = verify_jwt(token) {
                    info!("Token verified, proceeding with request");
                    let fut = self.service.call(req);
                    return Box::pin(async move {
                        let res = fut.await?;
                        Ok(res.map_into_boxed_body())
                    });
                } else {
                    info!("Token verification failed");
                }
            } else {
                info!("Authorization header not in Bearer format");
            }
        } else {
            info!("No Authorization header found");
        }

        info!("Unauthorized request");
        let response = req.into_response(HttpResponse::Unauthorized().finish().map_into_boxed_body());
        Box::pin(async { Ok(response) })
    }
}

