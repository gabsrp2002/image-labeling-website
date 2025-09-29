use image_labeling_website::database::{establish_connection, create_tables};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Image Labeling Website Backend");
    println!("Setting up database...");
    
    // Establish database connection
    let db = establish_connection().await?;
    println!("Database connected successfully!");
    
    // Create tables
    create_tables(&db).await?;
    println!("Database tables created successfully!");
    
    println!("Backend is ready!");
    println!("Run 'cargo test' to execute the test suite.");
    
    Ok(())
}