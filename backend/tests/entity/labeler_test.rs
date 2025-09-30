use image_labeling_website::repository::*;
use super::super::common::test_utils::setup_test_db;

#[tokio::test]
async fn test_labeler_operations() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = setup_test_db().await;
    
    // Test labeler creation
    let labeler = LabelerRepository::create(
        &test_db.connection,
        "test_labeler".to_string(),
        "hashed_password_456".to_string(),
    ).await?;
    
    assert_eq!(labeler.username, "test_labeler");
    
    // Test group creation
    let group = GroupRepository::create(
        &test_db.connection,
        "Test Group".to_string(),
        Some("Test description".to_string()),
    ).await?;
    
    // Test adding labeler to group
    LabelerRepository::add_to_group(&test_db.connection, labeler.id, group.id).await?;
    
    // Test getting labeler groups
    let labeler_groups = LabelerRepository::get_groups(&test_db.connection, labeler.id).await?;
    assert_eq!(labeler_groups.len(), 1);
    assert_eq!(labeler_groups[0].id, group.id);
    
    // Cleanup happens automatically when test_db goes out of scope
    Ok(())
}
