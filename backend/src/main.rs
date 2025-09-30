use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use image_labeling_website::database::{establish_connection, create_tables};
use image_labeling_website::routes::auth::login;
use image_labeling_website::routes::admin::labeler::{
    create_labeler, get_labeler, list_labelers, update_labeler, delete_labeler
};
use image_labeling_website::routes::admin::groups::list_groups;
use image_labeling_website::repository::AdminRepository;
use image_labeling_website::middleware::auth::AdminAuthMiddleware;
use dotenv::dotenv;
use bcrypt::hash;

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
    
    // Create default admin user if it doesn't exist
    println!("Setting up default admin user...");
    match AdminRepository::find_by_username(&db, "admin").await {
        Ok(Some(_)) => {
            println!("Admin user already exists, skipping creation.");
        }
        Ok(None) => {
            // Create admin user with username "admin" and password "admin"
            let password_hash = hash("admin", bcrypt::DEFAULT_COST)?;
            AdminRepository::create(&db, "admin".to_string(), password_hash).await?;
            println!("Default admin user created successfully! Username: admin, Password: admin");
        }
        Err(e) => {
            eprintln!("Error checking for existing admin user: {}", e);
            return Err(e.into());
        }
    }
    
    println!("Starting HTTP server on http://127.0.0.1:8080");
    
    // Start the HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/login", web::post().to(login))
                    .service(
                        web::scope("/admin")
                            .wrap(AdminAuthMiddleware)
                            .route("/groups", web::get().to(list_groups))
                            .service(
                                web::scope("/labeler")
                                    .route("", web::post().to(create_labeler))
                                    .route("", web::get().to(list_labelers))
                                    .route("/{id}", web::get().to(get_labeler))
                                    .route("/{id}", web::put().to(update_labeler))
                                    .route("/{id}", web::delete().to(delete_labeler))
                            )
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    
    Ok(())
}