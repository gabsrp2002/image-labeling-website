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
        Err(error) => {
            // Check if it's a server error (JWT generation, database issues, etc.)
            if error.contains("JWT_SECRET") || 
               error.contains("Token generation") || 
               error.contains("Database error") ||
               error.contains("Password verification") ||
               error.contains("Invalid timestamp") {
                Ok(HttpResponse::InternalServerError().json(format!("Server error: {}", error)))
            } else {
                // Client errors (invalid credentials, user not found, invalid role)
                Ok(HttpResponse::Forbidden().json(format!("Authentication failed: {}", error)))
            }
        }
    }
}
