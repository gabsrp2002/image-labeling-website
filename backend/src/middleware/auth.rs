use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use actix_web::body::EitherBody;
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use std::env;

use crate::schemas::auth::Claims;

pub struct AdminAuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AdminAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminAuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminAuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AdminAuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AdminAuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Extract the Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok());

            match auth_header {
                Some(header) => {
                    // Check if it starts with "Bearer "
                    if !header.starts_with("Bearer ") {
                        let response = HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "Invalid authorization header format"
                            }))
                            .map_into_right_body();
                        return Ok(ServiceResponse::new(req.into_parts().0, response));
                    }

                    // Extract the token
                    let token = &header[7..]; // Remove "Bearer " prefix

                    // Validate the JWT token
                    match Self::validate_jwt_token(token) {
                        Ok(claims) => {
                            // Check if the role is admin
                            if claims.role != "admin" {
                                let response = HttpResponse::Forbidden()
                                    .json(serde_json::json!({
                                        "error": "Insufficient permissions. Admin role required."
                                    }))
                                    .map_into_right_body();
                                return Ok(ServiceResponse::new(req.into_parts().0, response));
                            }

                            // Add user info to request extensions for use in handlers
                            req.extensions_mut().insert(claims);

                            // Continue with the request
                            let res = service.call(req).await?;
                            Ok(res.map_into_left_body())
                        }
                        Err(_) => {
                            let response = HttpResponse::Unauthorized()
                                .json(serde_json::json!({
                                    "error": "Invalid or expired token"
                                }))
                                .map_into_right_body();
                            Ok(ServiceResponse::new(req.into_parts().0, response))
                        }
                    }
                }
                None => {
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "error": "Authorization header missing"
                        }))
                        .map_into_right_body();
                    Ok(ServiceResponse::new(req.into_parts().0, response))
                }
            }
        })
    }
}

pub struct LabelerAuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LabelerAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = LabelerAuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LabelerAuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct LabelerAuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LabelerAuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Extract the Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok());

            match auth_header {
                Some(header) => {
                    // Check if it starts with "Bearer "
                    if !header.starts_with("Bearer ") {
                        let response = HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "Invalid authorization header format"
                            }))
                            .map_into_right_body();
                        return Ok(ServiceResponse::new(req.into_parts().0, response));
                    }

                    // Extract the token
                    let token = &header[7..]; // Remove "Bearer " prefix

                    // Validate the JWT token
                    match Self::validate_jwt_token(token) {
                        Ok(claims) => {
                            // Check if the role is labeler
                            if claims.role != "labeler" {
                                let response = HttpResponse::Forbidden()
                                    .json(serde_json::json!({
                                        "error": "Insufficient permissions. Labeler role required."
                                    }))
                                    .map_into_right_body();
                                return Ok(ServiceResponse::new(req.into_parts().0, response));
                            }

                            // Add user info to request extensions for use in handlers
                            req.extensions_mut().insert(claims);

                            // Continue with the request
                            let res = service.call(req).await?;
                            Ok(res.map_into_left_body())
                        }
                        Err(_) => {
                            let response = HttpResponse::Unauthorized()
                                .json(serde_json::json!({
                                    "error": "Invalid or expired token"
                                }))
                                .map_into_right_body();
                            Ok(ServiceResponse::new(req.into_parts().0, response))
                        }
                    }
                }
                None => {
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "error": "Authorization header missing"
                        }))
                        .map_into_right_body();
                    Ok(ServiceResponse::new(req.into_parts().0, response))
                }
            }
        })
    }
}

impl<S> LabelerAuthMiddlewareService<S> {
    fn validate_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| jsonwebtoken::errors::ErrorKind::InvalidToken)?;

        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &validation,
        )?;

        Ok(token_data.claims)
    }
}

impl<S> AdminAuthMiddlewareService<S> {
    fn validate_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| jsonwebtoken::errors::ErrorKind::InvalidToken)?;

        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &validation,
        )?;

        Ok(token_data.claims)
    }
}
