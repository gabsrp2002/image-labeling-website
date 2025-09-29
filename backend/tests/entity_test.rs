mod common;

use image_labeling_website::repository::*;
use common::test_utils::setup_test_db;

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
