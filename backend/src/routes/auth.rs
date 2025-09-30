use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;

use crate::schemas::auth::LoginRequest;
use crate::service::auth::AuthService;

pub async fn login(
    db: web::Data<DatabaseConnection>,
    login_request: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    match AuthService::login(&db, login_request.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(_) => Ok(HttpResponse::Forbidden().json("Invalid credentials")),
    }
}
