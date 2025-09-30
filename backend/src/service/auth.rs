use sea_orm::DatabaseConnection;
use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Duration, Utc};
use std::env;

use crate::repository::{AdminRepository, LabelerRepository};
use crate::schemas::auth::{LoginRequest, LoginResponse, Claims};

pub struct AuthService;

impl AuthService {
    
    pub async fn login(
        db: &DatabaseConnection,
        login_request: LoginRequest,
    ) -> Result<LoginResponse, String> {
        match login_request.role.as_str() {
            "admin" => Self::authenticate_admin(db, &login_request).await,
            "labeler" => Self::authenticate_labeler(db, &login_request).await,
            _ => Err("Invalid role".to_string()),
        }
    }

    async fn authenticate_admin(
        db: &DatabaseConnection,
        login_request: &LoginRequest,
    ) -> Result<LoginResponse, String> {
        let admin = AdminRepository::find_by_username(db, &login_request.username)
            .await
            .map_err(|_| "Database error".to_string())?;

        let admin = admin.ok_or("User not found".to_string())?;

        let is_valid = verify(&login_request.password, &admin.password_hash)
            .map_err(|_| "Password verification failed".to_string())?;

        if !is_valid {
            return Err("Invalid credentials".to_string());
        }

        let token = Self::generate_jwt(admin.id, "admin")?;
        Ok(LoginResponse { token })
    }

    async fn authenticate_labeler(
        db: &DatabaseConnection,
        login_request: &LoginRequest,
    ) -> Result<LoginResponse, String> {
        let labeler = LabelerRepository::find_by_username(db, &login_request.username)
            .await
            .map_err(|_| "Database error".to_string())?;

        let labeler = labeler.ok_or("User not found".to_string())?;

        let is_valid = verify(&login_request.password, &labeler.password_hash)
            .map_err(|_| "Password verification failed".to_string())?;

        if !is_valid {
            return Err("Invalid credentials".to_string());
        }

        let token = Self::generate_jwt(labeler.id, "labeler")?;
        Ok(LoginResponse { token })
    }

    fn generate_jwt(user_id: i32, role: &str) -> Result<String, String> {
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| "JWT_SECRET environment variable not set".to_string())?;

        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .ok_or("Invalid timestamp for JWT expiration".to_string())?
            .timestamp() as usize;

        let claims = Claims {
            user_id,
            role: role.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .map_err(|e| format!("Token generation failed: {}", e))
    }
}
