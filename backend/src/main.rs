use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use image_labeling_website::database::{establish_connection, create_tables};
use image_labeling_website::routes::auth::login;
use image_labeling_website::routes::admin::labeler::{
    create_labeler, get_labeler, list_labelers, update_labeler, delete_labeler
};
use image_labeling_website::routes::admin::groups::{list_groups, create_group, get_group_details, delete_group};
use image_labeling_website::routes::admin::image::{upload_image, get_image_details};
use image_labeling_website::routes::admin::tag::{
    create_tag, get_tag, list_tags_by_group, update_tag, delete_tag
};
use image_labeling_website::routes::admin::final_tags::{
    get_final_tags, update_final_tags, auto_generate_final_tags
};
use image_labeling_website::routes::admin::export::bulk_export;
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
                            .route("/groups", web::post().to(create_group))
                            .route("/groups/{id}", web::get().to(get_group_details))
                            .route("/groups/{id}", web::delete().to(delete_group))
                            .route("/groups/{group_id}/image/{image_id}", web::get().to(get_image_details))
                            .route("/image", web::post().to(upload_image))
                            .route("/image/{image_id}/final-tags", web::get().to(get_final_tags))
                            .route("/image/{image_id}/final-tags", web::put().to(update_final_tags))
                            .route("/image/{image_id}/final-tags/auto-generate", web::post().to(auto_generate_final_tags))
                            .route("/export/bulk", web::get().to(bulk_export))
                            .service(
                                web::scope("/tag")
                                    .route("", web::post().to(create_tag))
                                    .route("/{id}", web::get().to(get_tag))
                                    .route("/{id}", web::put().to(update_tag))
                                    .route("/{id}", web::delete().to(delete_tag))
                                    .route("/group/{group_id}", web::get().to(list_tags_by_group))
                            )
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