use image_labeling_website::repository::*;
use super::super::common::test_utils::setup_test_db;

#[tokio::test]
async fn test_tag_operations() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = setup_test_db().await;
    
    // Create a group first
    let group = GroupRepository::create(
        &test_db.connection,
        "Test Group".to_string(),
        Some("Test description".to_string()),
    ).await?;
    
    // Test tag creation
    let tag = TagRepository::create(
        &test_db.connection,
        "test_tag".to_string(),
        Some("Test tag description".to_string()),
        group.id,
    ).await?;
    
    assert_eq!(tag.name, "test_tag");
    assert_eq!(tag.group_id, group.id);
    
    // Test getting tags by group
    let group_tags = GroupRepository::get_possible_tags(&test_db.connection, group.id).await?;
    assert_eq!(group_tags.len(), 1);
    assert_eq!(group_tags[0].id, tag.id);
    
    // Cleanup happens automatically when test_db goes out of scope
    Ok(())
}
