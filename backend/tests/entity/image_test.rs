use image_labeling_website::repository::*;
use super::super::common::test_utils::setup_test_db;

#[tokio::test]
async fn test_image_operations() -> Result<(), Box<dyn std::error::Error>> {
    let test_db = setup_test_db().await;
    
    // Create a group first
    let group = GroupRepository::create(
        &test_db.connection,
        "Test Group".to_string(),
        Some("Test description".to_string()),
    ).await?;
    
    // Test image creation
    let image = ImageRepository::create(
        &test_db.connection,
        "/path/to/test.jpg".to_string(),
        "test.jpg".to_string(),
        group.id,
    ).await?;
    
    assert_eq!(image.path, "/path/to/test.jpg");
    assert_eq!(image.name, "test.jpg");
    assert_eq!(image.group_id, group.id);
    
    // Test getting images by group
    let group_images = GroupRepository::get_images(&test_db.connection, group.id).await?;
    assert_eq!(group_images.len(), 1);
    assert_eq!(group_images[0].id, image.id);
    
    // Cleanup happens automatically when test_db goes out of scope
    Ok(())
}
