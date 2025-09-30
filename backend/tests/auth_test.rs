mod common;

use image_labeling_website::service::auth::AuthService;
use image_labeling_website::schemas::auth::LoginRequest;
use image_labeling_website::repository::*;
use common::test_utils::setup_test_db;
use jsonwebtoken::{decode, DecodingKey, Validation};
use image_labeling_website::schemas::auth::Claims;
use bcrypt::hash;
use std::env;

#[tokio::test]
async fn test_admin_login_success() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Create test admin with hashed password
    let password = "test_password_123";
    let hashed_password = hash(password, bcrypt::DEFAULT_COST)?;
    
    let admin = AdminRepository::create(
        &test_db.connection,
        "test_admin".to_string(),
        hashed_password,
    ).await?;
    
    // Test login
    let login_request = LoginRequest {
        username: "test_admin".to_string(),
        password: password.to_string(),
        role: "admin".to_string(),
    };
    
    let result = AuthService::login(&test_db.connection, login_request).await;
    assert!(result.is_ok());
    
    let login_response = result.unwrap();
    assert!(!login_response.token.is_empty());
    
    // Verify JWT token
    let token_data = decode::<Claims>(
        &login_response.token,
        &DecodingKey::from_secret("test-secret-key-for-testing".as_ref()),
        &Validation::default(),
    )?;
    
    assert_eq!(token_data.claims.user_id, admin.id);
    assert_eq!(token_data.claims.role, "admin");
    
    // Note: No cleanup needed as each test uses its own JWT secret
    
    Ok(())
}

#[tokio::test]
async fn test_labeler_login_success() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Create test labeler with hashed password
    let password = "test_password_456";
    let hashed_password = hash(password, bcrypt::DEFAULT_COST)?;
    
    let labeler = LabelerRepository::create(
        &test_db.connection,
        "test_labeler".to_string(),
        hashed_password,
    ).await?;
    
    // Test login
    let login_request = LoginRequest {
        username: "test_labeler".to_string(),
        password: password.to_string(),
        role: "labeler".to_string(),
    };
    
    let result = AuthService::login(&test_db.connection, login_request).await;
    assert!(result.is_ok());
    
    let login_response = result.unwrap();
    assert!(!login_response.token.is_empty());
    
    // Verify JWT token
    let token_data = decode::<Claims>(
        &login_response.token,
        &DecodingKey::from_secret("test-secret-key-for-testing".as_ref()),
        &Validation::default(),
    )?;
    
    assert_eq!(token_data.claims.user_id, labeler.id);
    assert_eq!(token_data.claims.role, "labeler");
    
    // Note: No cleanup needed as each test uses its own JWT secret
    
    Ok(())
}

#[tokio::test]
async fn test_admin_login_wrong_password() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Create test admin with hashed password
    let password = "test_password_123";
    let hashed_password = hash(password, bcrypt::DEFAULT_COST)?;
    
    AdminRepository::create(
        &test_db.connection,
        "test_admin".to_string(),
        hashed_password,
    ).await?;
    
    // Test login with wrong password
    let login_request = LoginRequest {
        username: "test_admin".to_string(),
        password: "wrong_password".to_string(),
        role: "admin".to_string(),
    };
    
    let result = AuthService::login(&test_db.connection, login_request).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid credentials");
    
    // Note: No cleanup needed as each test uses its own JWT secret
    
    Ok(())
}

#[tokio::test]
async fn test_labeler_login_wrong_password() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Create test labeler with hashed password
    let password = "test_password_456";
    let hashed_password = hash(password, bcrypt::DEFAULT_COST)?;
    
    LabelerRepository::create(
        &test_db.connection,
        "test_labeler".to_string(),
        hashed_password,
    ).await?;
    
    // Test login with wrong password
    let login_request = LoginRequest {
        username: "test_labeler".to_string(),
        password: "wrong_password".to_string(),
        role: "labeler".to_string(),
    };
    
    let result = AuthService::login(&test_db.connection, login_request).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid credentials");
    
    // Note: No cleanup needed as each test uses its own JWT secret
    
    Ok(())
}

#[tokio::test]
async fn test_login_user_not_found() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Test login with non-existent user
    let login_request = LoginRequest {
        username: "nonexistent_user".to_string(),
        password: "any_password".to_string(),
        role: "admin".to_string(),
    };
    
    let result = AuthService::login(&test_db.connection, login_request).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User not found");
    
    // Note: No cleanup needed as each test uses its own JWT secret
    
    Ok(())
}

#[tokio::test]
async fn test_login_invalid_role() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Test login with invalid role
    let login_request = LoginRequest {
        username: "test_user".to_string(),
        password: "password".to_string(),
        role: "invalid_role".to_string(),
    };
    
    let result = AuthService::login(&test_db.connection, login_request).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid role");
    
    // Note: No cleanup needed as each test uses its own JWT secret
    
    Ok(())
}

#[tokio::test]
async fn test_jwt_token_expiration() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Create test admin
    let password = "test_password_123";
    let hashed_password = hash(password, bcrypt::DEFAULT_COST)?;
    
    let _admin = AdminRepository::create(
        &test_db.connection,
        "test_admin".to_string(),
        hashed_password,
    ).await?;
    
    // Test login
    let login_request = LoginRequest {
        username: "test_admin".to_string(),
        password: password.to_string(),
        role: "admin".to_string(),
    };
    
    let result = AuthService::login(&test_db.connection, login_request).await;
    assert!(result.is_ok());
    
    let login_response = result.unwrap();
    
    // Verify JWT token has expiration set
    let token_data = decode::<Claims>(
        &login_response.token,
        &DecodingKey::from_secret("test-secret-key-for-testing".as_ref()),
        &Validation::default(),
    )?;
    
    // Check that expiration is in the future (within 24 hours)
    let now = chrono::Utc::now().timestamp() as usize;
    assert!(token_data.claims.exp > now);
    assert!(token_data.claims.exp <= now + 24 * 60 * 60); // 24 hours in seconds
    
    // Note: No cleanup needed as each test uses its own JWT secret
    
    Ok(())
}

#[tokio::test]
async fn test_admin_wrong_role() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Create test admin
    let password = "test_password_123";
    let hashed_password = hash(password, bcrypt::DEFAULT_COST)?;
    
    AdminRepository::create(
        &test_db.connection,
        "test_admin".to_string(),
        hashed_password,
    ).await?;
    
    // Try to login as admin but specify labeler role
    let login_request = LoginRequest {
        username: "test_admin".to_string(),
        password: password.to_string(),
        role: "labeler".to_string(),
    };
    
    let result = AuthService::login(&test_db.connection, login_request).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User not found"); // Admin not found in labeler table
    
    Ok(())
}

#[tokio::test]
async fn test_labeler_wrong_role() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Create test labeler
    let password = "test_password_456";
    let hashed_password = hash(password, bcrypt::DEFAULT_COST)?;
    
    LabelerRepository::create(
        &test_db.connection,
        "test_labeler".to_string(),
        hashed_password,
    ).await?;
    
    // Try to login as labeler but specify admin role
    let login_request = LoginRequest {
        username: "test_labeler".to_string(),
        password: password.to_string(),
        role: "admin".to_string(),
    };
    
    let result = AuthService::login(&test_db.connection, login_request).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User not found"); // Labeler not found in admin table
    
    Ok(())
}
