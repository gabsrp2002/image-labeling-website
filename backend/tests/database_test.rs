mod common;

use image_labeling_website::repository::*;
use crate::common::test_utils::setup_test_db;

#[tokio::test]
async fn test_database_setup() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up database...");
    
    // Setup test database - cleanup happens automatically via Drop trait
    let test_db = setup_test_db().await;
    println!("Database connected successfully!");
    
    // Test the database by creating some sample data
    test_database_operations(&test_db.connection).await?;
    
    println!("Database test completed successfully!");
    
    // Cleanup happens automatically when test_db goes out of scope
    Ok(())
}

async fn test_database_operations(db: &sea_orm::DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing database operations...");
    
    // Create a test group
    let group = GroupRepository::create(
        db,
        "Test Group".to_string(),
        Some("A test group for image labeling".to_string()),
    ).await?;
    println!("Created group: {:?}", group);
    
    // Create a test admin
    let admin = AdminRepository::create(
        db,
        "admin".to_string(),
        "hashed_password".to_string(),
    ).await?;
    println!("Created admin: {:?}", admin);
    
    // Create a test labeler
    let labeler = LabelerRepository::create(
        db,
        "labeler1".to_string(),
        "hashed_password".to_string(),
    ).await?;
    println!("Created labeler: {:?}", labeler);
    
    // Add labeler to group
    LabelerRepository::add_to_group(db, labeler.id, group.id).await?;
    println!("Added labeler to group");
    
    // Create test tags
    let tag = TagRepository::create(
        db,
        "cat".to_string(),
        Some("A feline animal".to_string()),
        group.id,
    ).await?;
    println!("Created tag: {:?}", tag);
    
    let tag2 = TagRepository::create(
        db,
        "dog".to_string(),
        Some("A canine animal".to_string()),
        group.id,
    ).await?;
    println!("Created second tag: {:?}", tag2);
    
    let tag3 = TagRepository::create(
        db,
        "outdoor".to_string(),
        Some("Taken outdoors".to_string()),
        group.id,
    ).await?;
    println!("Created third tag: {:?}", tag3);
    
    // Create a test image
    let image = ImageRepository::create(
        db,
        "/path/to/test/image.jpg".to_string(),
        "test_image.jpg".to_string(),
        group.id,
    ).await?;
    println!("Created image: {:?}", image);
    
    // Create multiple tag assignments for the same image by the same labeler
    let image_tags = ImageTagsRepository::assign_multiple_tags(
        db,
        image.id,
        labeler.id,
        vec![tag.id, tag2.id, tag3.id],
    ).await?;
    println!("Created multiple image tag assignments: {:?}", image_tags);
    
    // Test queries
    let groups = GroupRepository::get_all(db).await?;
    println!("All groups: {:?}", groups);
    
    let labeler_groups = LabelerRepository::get_groups(db, labeler.id).await?;
    println!("Labeler groups: {:?}", labeler_groups);
    
    let group_images = GroupRepository::get_images(db, group.id).await?;
    println!("Group images: {:?}", group_images);
    
    let group_tags = GroupRepository::get_possible_tags(db, group.id).await?;
    println!("Group tags: {:?}", group_tags);
    
    // Test multiple tags functionality
    let labeler_tags_for_image = ImageTagsRepository::get_tags_for_image_by_labeler(db, image.id, labeler.id).await?;
    println!("Tags assigned by labeler to image: {:?}", labeler_tags_for_image);
    
    let all_tags_for_image = ImageTagsRepository::get_all_tags_for_image(db, image.id).await?;
    println!("All tags assigned to image: {:?}", all_tags_for_image);
    
    println!("Database test completed successfully!");
    Ok(())
}
