use sea_orm::*;
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:sqlite.db".to_string());
    
    Database::connect(&database_url).await
}

pub async fn establish_connection_with_url(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}

pub async fn create_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Create all tables using raw SQL
    let create_admin_table = r#"
        CREATE TABLE IF NOT EXISTS admin (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        )
    "#;
    
    let create_labeler_table = r#"
        CREATE TABLE IF NOT EXISTS labeler (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        )
    "#;
    
    let create_group_table = r#"
        CREATE TABLE IF NOT EXISTS "group" (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT
        )
    "#;
    
    let create_image_table = r#"
        CREATE TABLE IF NOT EXISTS image (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            name TEXT NOT NULL,
            group_id INTEGER NOT NULL,
            FOREIGN KEY (group_id) REFERENCES "group"(id)
        )
    "#;
    
    let create_tag_table = r#"
        CREATE TABLE IF NOT EXISTS tag (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            group_id INTEGER NOT NULL,
            FOREIGN KEY (group_id) REFERENCES "group"(id)
        )
    "#;
    
    let create_image_tags_table = r#"
        CREATE TABLE IF NOT EXISTS image_tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            image_id INTEGER NOT NULL,
            labeler_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            created_at DATETIME NOT NULL,
            FOREIGN KEY (image_id) REFERENCES image(id),
            FOREIGN KEY (labeler_id) REFERENCES labeler(id),
            FOREIGN KEY (tag_id) REFERENCES tag(id),
            UNIQUE(image_id, labeler_id, tag_id)
        )
    "#;
    
    let create_labeler_groups_table = r#"
        CREATE TABLE IF NOT EXISTS labeler_groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            labeler_id INTEGER NOT NULL,
            group_id INTEGER NOT NULL,
            FOREIGN KEY (labeler_id) REFERENCES labeler(id),
            FOREIGN KEY (group_id) REFERENCES "group"(id)
        )
    "#;
    
    // Execute table creation statements
    db.execute_unprepared(create_admin_table).await?;
    db.execute_unprepared(create_labeler_table).await?;
    db.execute_unprepared(create_group_table).await?;
    db.execute_unprepared(create_image_table).await?;
    db.execute_unprepared(create_tag_table).await?;
    db.execute_unprepared(create_image_tags_table).await?;
    db.execute_unprepared(create_labeler_groups_table).await?;
    
    Ok(())
}
