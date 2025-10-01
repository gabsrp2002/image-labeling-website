use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::{
    repository::{FinalTagsRepository, ImageTagsRepository, TagRepository},
};

#[derive(Serialize)]
pub struct FinalTagsResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Vec<FinalTagData>>,
}

#[derive(Serialize)]
pub struct FinalTagData {
    pub id: i32,
    pub tag_id: i32,
    pub tag_name: String,
    pub is_admin_override: bool,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct UpdateFinalTagsRequest {
    pub tag_ids: Vec<i32>,
}

pub async fn get_final_tags(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let image_id = path.into_inner();
    match FinalTagsRepository::get_by_image(&db, image_id).await {
        Ok(final_tags) => {
            let mut final_tag_data = Vec::new();
            
            for final_tag in final_tags {
                // Get tag name
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
                    Ok(None) => {
                        // Tag not found, skip this final tag
                        continue;
                    }
                    Err(_) => {
                        return Ok(HttpResponse::InternalServerError().json(FinalTagsResponse {
                            success: false,
                            message: "Database error".to_string(),
                            data: None,
                        }));
                    }
                }
            }
            
            Ok(HttpResponse::Ok().json(FinalTagsResponse {
                success: true,
                message: "Final tags retrieved successfully".to_string(),
                data: Some(final_tag_data),
            }))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json(FinalTagsResponse {
            success: false,
            message: "Database error".to_string(),
            data: None,
        })),
    }
}

pub async fn update_final_tags(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i32>,
    request: web::Json<UpdateFinalTagsRequest>,
) -> Result<HttpResponse> {
    let image_id = path.into_inner();
    match FinalTagsRepository::replace_final_tags(&db, image_id, request.tag_ids.clone(), true).await {
        Ok(final_tags) => {
            let mut final_tag_data = Vec::new();
            
            for final_tag in final_tags {
                // Get tag name
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
                    Ok(None) => {
                        // Tag not found, skip this final tag
                        continue;
                    }
                    Err(_) => {
                        return Ok(HttpResponse::InternalServerError().json(FinalTagsResponse {
                            success: false,
                            message: "Database error".to_string(),
                            data: None,
                        }));
                    }
                }
            }
            
            Ok(HttpResponse::Ok().json(FinalTagsResponse {
                success: true,
                message: "Final tags updated successfully".to_string(),
                data: Some(final_tag_data),
            }))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json(FinalTagsResponse {
            success: false,
            message: "Database error".to_string(),
            data: None,
        })),
    }
}

pub async fn auto_generate_final_tags(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let image_id = path.into_inner();
    // Get all image tags for this image
    match ImageTagsRepository::get_all_tags_for_image(&db, image_id).await {
        Ok(image_tags) => {
            if image_tags.is_empty() {
                return Ok(HttpResponse::Ok().json(FinalTagsResponse {
                    success: true,
                    message: "No tags found for this image".to_string(),
                    data: Some(vec![]),
                }));
            }
            
            // Group tags by tag_id and count occurrences
            use std::collections::HashMap;
            let mut tag_counts: HashMap<i32, i32> = HashMap::new();
            
            for image_tag in &image_tags {
                *tag_counts.entry(image_tag.tag_id).or_insert(0) += 1;
            }
            
            // Get total number of labelers who tagged this image
            let total_labelers = image_tags.iter()
                .map(|it| it.labeler_id)
                .collect::<std::collections::HashSet<_>>()
                .len() as i32;
            
            // Find tags chosen by at least 50% of labelers
            let threshold = (total_labelers as f64 * 0.5).ceil() as i32;
            let final_tag_ids: Vec<i32> = tag_counts
                .into_iter()
                .filter(|(_, count)| *count >= threshold)
                .map(|(tag_id, _)| tag_id)
                .collect();
            
            // Replace final tags with auto-generated ones
            match FinalTagsRepository::replace_final_tags(&db, image_id, final_tag_ids, false).await {
                Ok(final_tags) => {
                    let mut final_tag_data = Vec::new();
                    
                    for final_tag in final_tags {
                        // Get tag name
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
                            Ok(None) => {
                                // Tag not found, skip this final tag
                                continue;
                            }
                            Err(_) => {
                                return Ok(HttpResponse::InternalServerError().json(FinalTagsResponse {
                                    success: false,
                                    message: "Database error".to_string(),
                                    data: None,
                                }));
                            }
                        }
                    }
                    
                    Ok(HttpResponse::Ok().json(FinalTagsResponse {
                        success: true,
                        message: "Final tags auto-generated successfully".to_string(),
                        data: Some(final_tag_data),
                    }))
                }
                Err(_) => Ok(HttpResponse::InternalServerError().json(FinalTagsResponse {
                    success: false,
                    message: "Database error".to_string(),
                    data: None,
                })),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json(FinalTagsResponse {
            success: false,
            message: "Database error".to_string(),
            data: None,
        })),
    }
}
