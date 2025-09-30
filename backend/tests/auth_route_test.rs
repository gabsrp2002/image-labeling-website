mod common;

use actix_web::{web, App, test, http::StatusCode};
use image_labeling_website::routes::auth::login;
use image_labeling_website::repository::*;
use common::test_utils::setup_test_db;
use bcrypt::hash;
use std::env;

#[tokio::test]
async fn test_login_route_admin_success() -> Result<(), Box<dyn std::error::Error>> {
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
    
    // Set up the app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(test_db.connection.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/login", web::post().to(login))
            )
    ).await;
    
    // Create login request
    let login_request = serde_json::json!({
        "username": "test_admin",
        "password": "test_password_123",
        "role": "admin"
    });
    
    // Make the request
    let req = test::TestRequest::post()
        .uri("/api/v1/login")
        .set_json(&login_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assertions
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["token"].is_string());
    assert!(!body["token"].as_str().unwrap().is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_login_route_labeler_success() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Create test labeler
    let password = "test_password_456";
    let hashed_password = hash(password, bcrypt::DEFAULT_COST)?;
    
    let _labeler = LabelerRepository::create(
        &test_db.connection,
        "test_labeler".to_string(),
        hashed_password,
    ).await?;
    
    // Set up the app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(test_db.connection.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/login", web::post().to(login))
            )
    ).await;
    
    // Create login request
    let login_request = serde_json::json!({
        "username": "test_labeler",
        "password": "test_password_456",
        "role": "labeler"
    });
    
    // Make the request
    let req = test::TestRequest::post()
        .uri("/api/v1/login")
        .set_json(&login_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assertions
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["token"].is_string());
    assert!(!body["token"].as_str().unwrap().is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_login_route_invalid_credentials() -> Result<(), Box<dyn std::error::Error>> {
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
    
    // Set up the app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(test_db.connection.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/login", web::post().to(login))
            )
    ).await;
    
    // Create login request with wrong password
    let login_request = serde_json::json!({
        "username": "test_admin",
        "password": "wrong_password",
        "role": "admin"
    });
    
    // Make the request
    let req = test::TestRequest::post()
        .uri("/api/v1/login")
        .set_json(&login_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assertions
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    
    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Authentication failed"));
    assert!(body_str.contains("Invalid credentials"));
    
    Ok(())
}

#[tokio::test]
async fn test_login_route_user_not_found() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Set up the app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(test_db.connection.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/login", web::post().to(login))
            )
    ).await;
    
    // Create login request for non-existent user
    let login_request = serde_json::json!({
        "username": "nonexistent_user",
        "password": "any_password",
        "role": "admin"
    });
    
    // Make the request
    let req = test::TestRequest::post()
        .uri("/api/v1/login")
        .set_json(&login_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assertions
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    
    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Authentication failed"));
    assert!(body_str.contains("User not found"));
    
    Ok(())
}

#[tokio::test]
async fn test_login_route_invalid_role() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Set up the app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(test_db.connection.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/login", web::post().to(login))
            )
    ).await;
    
    // Create login request with invalid role
    let login_request = serde_json::json!({
        "username": "test_user",
        "password": "password",
        "role": "invalid_role"
    });
    
    // Make the request
    let req = test::TestRequest::post()
        .uri("/api/v1/login")
        .set_json(&login_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assertions
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    
    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Authentication failed"));
    assert!(body_str.contains("Invalid role"));
    
    Ok(())
}

#[tokio::test]
async fn test_login_route_malformed_json() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Set up the app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(test_db.connection.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/login", web::post().to(login))
            )
    ).await;
    
    // Make the request with malformed JSON
    let req = test::TestRequest::post()
        .uri("/api/v1/login")
        .insert_header(("content-type", "application/json"))
        .set_payload("{ invalid json }")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assertions - should return 400 for bad request
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
    Ok(())
}

#[tokio::test]
async fn test_login_route_missing_fields() -> Result<(), Box<dyn std::error::Error>> {
    // Set up test environment
    unsafe { env::set_var("JWT_SECRET", "test-secret-key-for-testing"); }
    let test_db = setup_test_db().await;
    
    // Set up the app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(test_db.connection.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/login", web::post().to(login))
            )
    ).await;
    
    // Create login request with missing fields
    let login_request = serde_json::json!({
        "username": "test_admin"
        // Missing password and role
    });
    
    // Make the request
    let req = test::TestRequest::post()
        .uri("/api/v1/login")
        .set_json(&login_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assertions - should return 400 for bad request
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
    Ok(())
}
