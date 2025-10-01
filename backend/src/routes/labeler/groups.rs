use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use sea_orm::DatabaseConnection;
use crate::repository::LabelerRepository;
use crate::schemas::auth::Claims;

#[derive(serde::Serialize)]
pub struct GroupResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ImageResponse {
    pub id: i32,
    pub filename: String,
    pub status: String, // "done" or "pending"
}

pub async fn get_groups(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract user info from request extensions (set by middleware)
    let claims = req.extensions().get::<Claims>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("User not authenticated"))?
        .clone();

    let labeler_id = claims.user_id;
    
    match LabelerRepository::get_groups(&db, labeler_id).await {
        Ok(groups) => {
            let response: Vec<GroupResponse> = groups
                .into_iter()
                .map(|group| GroupResponse {
                    id: group.id,
                    name: group.name,
                    description: group.description,
                })
                .collect();
            
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            eprintln!("Error fetching groups for labeler {}: {}", labeler_id, e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch groups"
            })))
        }
    }
}

pub async fn get_group_images(
    req: HttpRequest,
    path: web::Path<i32>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract user info from request extensions (set by middleware)
    let claims = req.extensions().get::<Claims>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("User not authenticated"))?
        .clone();

    let labeler_id = claims.user_id;
    let group_id = path.into_inner();
    
    // First verify that the labeler is part of this group
    match LabelerRepository::get_groups(&db, labeler_id).await {
        Ok(groups) => {
            if !groups.iter().any(|group| group.id == group_id) {
                return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                    "error": "You are not authorized to access this group"
                })));
            }
        }
        Err(e) => {
            eprintln!("Error verifying group access for labeler {}: {}", labeler_id, e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to verify group access"
            })));
        }
    }
    
    // Get images for the group
    match crate::repository::GroupRepository::get_images(&db, group_id).await {
        Ok(images) => {
            // For each image, check if the labeler has tagged it
            let mut image_responses = Vec::new();
            
            for image in images {
                // Check if this labeler has any tags for this image
                let has_tags = match crate::repository::ImageTagsRepository::get_by_image_and_labeler(
                    &db, 
                    image.id, 
                    labeler_id
                ).await {
                    Ok(tags) => !tags.is_empty(),
                    Err(_) => false,
                };
                
                let status = if has_tags { "done" } else { "pending" };
                
                image_responses.push(ImageResponse {
                    id: image.id,
                    filename: image.filename,
                    status: status.to_string(),
                });
            }
            
            Ok(HttpResponse::Ok().json(image_responses))
        }
        Err(e) => {
            eprintln!("Error fetching images for group {}: {}", group_id, e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch images"
            })))
        }
    }
}
