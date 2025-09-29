use image_labeling_website::database::{establish_connection_with_url, create_tables};
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Global counter for unique database names
static COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct TestDatabase {
    pub connection: sea_orm::DatabaseConnection,
    pub db_name: String,
}

impl TestDatabase {
    pub async fn new() -> Self {
        // Generate unique database filename using counter + timestamp + random
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let random_part = fastrand::u64(..);
        let db_name = format!("test_{}_{}_{}.db", counter, timestamp, random_part);
        
        // Remove existing test database if it exists
        let _ = fs::remove_file(&db_name);
        
        // Create the database file with proper permissions
        let file = fs::File::create(&db_name).expect("Failed to create database file");
        drop(file); // Close the file handle immediately
        
        // Small delay to ensure file system operations complete
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        
        // Establish connection with specific database URL (no environment variable)
        let database_url = format!("sqlite:{}", db_name);
        let connection = establish_connection_with_url(&database_url).await.expect("Failed to connect to test database");
        create_tables(&connection).await.expect("Failed to create test tables");
        
        Self { connection, db_name }
    }
}

// Implement Drop to ensure cleanup happens even if test fails
impl Drop for TestDatabase {
    fn drop(&mut self) {
        // Close the connection first
        drop(std::mem::take(&mut self.connection));
        
        // Remove the database file
        let _ = fs::remove_file(&self.db_name);
    }
}

// Convenience function for backward compatibility
pub async fn setup_test_db() -> TestDatabase {
    TestDatabase::new().await
}
