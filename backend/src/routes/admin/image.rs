use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use crate::schemas::admin::{ApiResponse, UploadImageRequest};
use crate::service::admin::AdminService;
use crate::repository::{ImageRepository, ImageTagsRepository, TagRepository, FinalTagsRepository, GroupRepository};

#[derive(Serialize)]
pub struct ImageDetailsResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<ImageDetailsData>,
}

#[derive(Serialize)]
pub struct ImageDetailsData {
    pub image: ImageData,
    pub tag_statistics: Vec<TagStatistic>,
    pub final_tags: Vec<FinalTagData>,
    pub has_admin_override: bool,
}

#[derive(Serialize)]
pub struct ImageData {
    pub id: i32,
    pub filename: String,
    pub filetype: String,
    pub base64_data: String,
    pub uploaded_at: String,
}

#[derive(Serialize)]
pub struct TagStatistic {
    pub tag_id: i32,
    pub tag_name: String,
    pub percentage: f64,
    pub count: i32,
    pub total_labelers: i32,
}

#[derive(Serialize)]
pub struct FinalTagData {
    pub id: i32,
    pub tag_id: i32,
    pub tag_name: String,
    pub is_admin_override: bool,
    pub created_at: String,
}

pub async fn upload_image(
    db: web::Data<DatabaseConnection>,
    request: web::Json<UploadImageRequest>,
) -> Result<HttpResponse> {
    match AdminService::upload_image(&db, request.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

pub async fn get_image_details(
    db: web::Data<DatabaseConnection>,
    path: web::Path<(i32, i32)>, // (group_id, image_id)
) -> Result<HttpResponse> {
    let (group_id, image_id) = path.into_inner();
    
    // Get image details
    let image = match ImageRepository::find_by_id(&db, image_id).await {
        Ok(Some(img)) => img,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ImageDetailsResponse {
                success: false,
                message: "Image not found".to_string(),
                data: None,
            }));
        }
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(ImageDetailsResponse {
                success: false,
                message: "Database error".to_string(),
                data: None,
            }));
        }
    };
    
    // Verify image belongs to the group
    if image.group_id != group_id {
        return Ok(HttpResponse::NotFound().json(ImageDetailsResponse {
            success: false,
            message: "Image not found in this group".to_string(),
            data: None,
        }));
    }
    
    // Get all image tags for this image
    let image_tags = match ImageTagsRepository::get_all_tags_for_image(&db, image_id).await {
        Ok(tags) => tags,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(ImageDetailsResponse {
                success: false,
                message: "Database error".to_string(),
                data: None,
            }));
        }
    };
    
    // Calculate tag statistics
    use std::collections::HashMap;
    let mut tag_counts: HashMap<i32, i32> = HashMap::new();
    
    for image_tag in &image_tags {
        *tag_counts.entry(image_tag.tag_id).or_insert(0) += 1;
    }
    
    // Get total number of unique labelers who tagged this image
    let total_labelers = image_tags.iter()
        .map(|it| it.labeler_id)
        .collect::<std::collections::HashSet<_>>()
        .len() as i32;
    
    // Get all possible tags for this group
    let all_group_tags = match GroupRepository::get_possible_tags(&db, group_id).await {
        Ok(tags) => tags,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(ImageDetailsResponse {
                success: false,
                message: "Database error".to_string(),
                data: None,
            }));
        }
    };
    
    // Build tag statistics for ALL group tags (including unused ones)
    let mut tag_statistics = Vec::new();
    for tag in all_group_tags {
        let count = tag_counts.get(&tag.id).copied().unwrap_or(0);
        let percentage = if total_labelers > 0 {
            (count as f64 / total_labelers as f64) * 100.0
        } else {
            0.0
        };
        
        tag_statistics.push(TagStatistic {
            tag_id: tag.id,
            tag_name: tag.name,
            percentage,
            count,
            total_labelers,
        });
    }
    
    // Sort by percentage descending
    tag_statistics.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());
    
    // Get final tags
    let final_tags = match FinalTagsRepository::get_by_image(&db, image_id).await {
        Ok(tags) => {
            let mut final_tag_data = Vec::new();
            for final_tag in tags {
                match TagRepository::find_by_id(&db, final_tag.tag_id).await {
                    Ok(Some(tag)) => {
                        final_tag_data.push(FinalTagData {
                            id: final_tag.id,
                            tag_id: final_tag.tag_id,
                            tag_name: tag.name,
                            is_admin_override: final_tag.is_admin_override,
                            created_at: final_tag.created_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                        });
                    }
                    Ok(None) => continue,
                    Err(_) => continue,
                }
            }
            final_tag_data
        }
        Err(_) => Vec::new(),
    };
    
    // Check if there's an admin override
    let has_admin_override = FinalTagsRepository::has_admin_override(&db, image_id).await.unwrap_or_default();
    
    let image_data = ImageData {
        id: image.id,
        filename: image.filename,
        filetype: image.filetype,
        base64_data: image.base64_data,
        uploaded_at: image.uploaded_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
    };
    
    Ok(HttpResponse::Ok().json(ImageDetailsResponse {
        success: true,
        message: "Image details retrieved successfully".to_string(),
        data: Some(ImageDetailsData {
            image: image_data,
            tag_statistics,
            final_tags,
            has_admin_override,
        }),
    }))
}
