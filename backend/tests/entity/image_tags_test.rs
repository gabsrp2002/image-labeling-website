use image_labeling_website::repository::*;
use super::super::common::test_utils::setup_test_db;

#[tokio::test]
async fn test_multiple_tags_assignment() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = setup_test_db().await;
    
    // Create test data
    let group = GroupRepository::create(
        &test_db.connection,
        "Test Group".to_string(),
        Some("Test description".to_string()),
    ).await?;
    
    let labeler = LabelerRepository::create(
        &test_db.connection,
        "test_labeler".to_string(),
        "hashed_password".to_string(),
    ).await?;
    
    let image = ImageRepository::create(
        &test_db.connection,
        "/path/to/test.jpg".to_string(),
        "test.jpg".to_string(),
        group.id,
    ).await?;
    
    let tag1 = TagRepository::create(
        &test_db.connection,
        "tag1".to_string(),
        Some("First tag".to_string()),
        group.id,
    ).await?;
    
    let tag2 = TagRepository::create(
        &test_db.connection,
        "tag2".to_string(),
        Some("Second tag".to_string()),
        group.id,
    ).await?;
    
    let tag3 = TagRepository::create(
        &test_db.connection,
        "tag3".to_string(),
        Some("Third tag".to_string()),
        group.id,
    ).await?;
    
    // Test multiple tag assignment
    let image_tags = ImageTagsRepository::assign_multiple_tags(
        &test_db.connection,
        image.id,
        labeler.id,
        vec![tag1.id, tag2.id, tag3.id],
    ).await?;
    
    assert_eq!(image_tags.len(), 3);
    
    // Test getting tags for image by labeler
    let labeler_tags = ImageTagsRepository::get_tags_for_image_by_labeler(&test_db.connection, image.id, labeler.id).await?;
    assert_eq!(labeler_tags.len(), 3);
    
    // Test getting all tags for image
    let all_tags = ImageTagsRepository::get_all_tags_for_image(&test_db.connection, image.id).await?;
    assert_eq!(all_tags.len(), 3);
    
    // Cleanup happens automatically when test_db goes out of scope
    Ok(())
}
