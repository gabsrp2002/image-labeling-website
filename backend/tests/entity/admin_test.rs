use image_labeling_website::repository::*;
use super::super::common::test_utils::setup_test_db;

#[tokio::test]
async fn test_admin_operations() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = setup_test_db().await;
    
    // Test admin creation
    let admin = AdminRepository::create(
        &test_db.connection,
        "test_admin".to_string(),
        "hashed_password_123".to_string(),
    ).await?;
    
    assert_eq!(admin.username, "test_admin");
    assert_eq!(admin.password_hash, "hashed_password_123");
    
    // Test admin lookup
    let found_admin = AdminRepository::find_by_username(&test_db.connection, "test_admin").await?;
    assert!(found_admin.is_some());
    assert_eq!(found_admin.unwrap().id, admin.id);
    
    // Cleanup happens automatically when test_db goes out of scope
    Ok(())
}
