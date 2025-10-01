use sea_orm::DatabaseConnection;
use crate::repository::{LabelerRepository, GroupRepository, ImageTagsRepository};
use crate::schemas::labeler::{
    GroupResponse, ImageResponse, GroupListResponse, ImageListResponse, ApiResponse
};

pub struct LabelerService;

impl LabelerService {
    pub async fn get_labeler_groups(
        db: &DatabaseConnection,
        labeler_id: i32,
    ) -> Result<ApiResponse<GroupListResponse>, String> {
        match LabelerRepository::get_groups(db, labeler_id).await {
            Ok(groups) => {
                let group_responses: Vec<GroupResponse> = groups
                    .into_iter()
                    .map(|group| GroupResponse {
                        id: group.id,
                        name: group.name,
                        description: group.description,
                    })
                    .collect();
                
                Ok(ApiResponse {
                    success: true,
                    message: "Groups retrieved successfully".to_string(),
                    data: Some(GroupListResponse {
                        groups: group_responses,
                    }),
                })
            }
            Err(e) => {
                eprintln!("Error fetching groups for labeler {}: {}", labeler_id, e);
                Err(format!("Failed to fetch groups: {}", e))
            }
        }
    }

    pub async fn get_group_images(
        db: &DatabaseConnection,
        labeler_id: i32,
        group_id: i32,
    ) -> Result<ApiResponse<ImageListResponse>, String> {
        // First verify that the labeler is part of this group
        match LabelerRepository::get_groups(db, labeler_id).await {
            Ok(groups) => {
                if !groups.iter().any(|group| group.id == group_id) {
                    return Err("You are not authorized to access this group".to_string());
                }
            }
            Err(e) => {
                eprintln!("Error verifying group access for labeler {}: {}", labeler_id, e);
                return Err(format!("Failed to verify group access: {}", e));
            }
        }
        
        // Get images for the group
        match GroupRepository::get_images(db, group_id).await {
            Ok(images) => {
                // For each image, check if the labeler has tagged it
                let mut image_responses = Vec::new();
                
                for image in images {
                    // Check if this labeler has any tags for this image
                    let has_tags = match ImageTagsRepository::get_by_image_and_labeler(
                        db, 
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
                
                Ok(ApiResponse {
                    success: true,
                    message: "Images retrieved successfully".to_string(),
                    data: Some(ImageListResponse {
                        images: image_responses,
                    }),
                })
            }
            Err(e) => {
                eprintln!("Error fetching images for group {}: {}", group_id, e);
                Err(format!("Failed to fetch images: {}", e))
            }
        }
    }
}
