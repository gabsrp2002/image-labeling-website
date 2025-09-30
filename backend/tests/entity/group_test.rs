use image_labeling_website::repository::*;
use super::super::common::test_utils::setup_test_db;

#[tokio::test]
async fn test_group_operations() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = setup_test_db().await;
    
    // Test group creation
    let group = GroupRepository::create(
        &test_db.connection,
        "Test Group".to_string(),
        Some("Test description".to_string()),
    ).await?;
    
    assert_eq!(group.name, "Test Group");
    assert_eq!(group.description, Some("Test description".to_string()));
    
    // Test group update
    let updated_group = GroupRepository::update(
        &test_db.connection,
        group.id,
        Some("Updated Group Name".to_string()),
        Some("Updated description".to_string()),
    ).await?;
    
    assert_eq!(updated_group.name, "Updated Group Name");
    assert_eq!(updated_group.description, Some("Updated description".to_string()));
    
    // Cleanup happens automatically when test_db goes out of scope
    Ok(())
}
