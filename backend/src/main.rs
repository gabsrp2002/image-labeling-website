use actix_web::{web, App, HttpServer};
use image_labeling_website::database::{establish_connection, create_tables};
use image_labeling_website::routes::auth::login;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    println!("Image Labeling Website Backend");
    println!("Setting up database...");
    
    // Establish database connection
    let db = establish_connection().await?;
    println!("Database connected successfully!");
    
    // Create tables
    create_tables(&db).await?;
    println!("Database tables created successfully!");
    
    println!("Starting HTTP server on http://127.0.0.1:8080");
    
    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(
                web::scope("/api")
                    .route("/login", web::post().to(login))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    
    Ok(())
}