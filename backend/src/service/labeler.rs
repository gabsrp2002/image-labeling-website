use sea_orm::DatabaseConnection;
use crate::repository::{LabelerRepository, GroupRepository, ImageTagsRepository};
use crate::schemas::labeler::{
    GroupResponse, ImageResponse, GroupListResponse, ImageListResponse, ApiResponse,
    TagResponse, ImageDetailResponse, UpdateImageTagsRequest, SuggestTagsResponse
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
                        base64_data: image.base64_data,
                        filetype: image.filetype,
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

    pub async fn get_image_details(
        db: &DatabaseConnection,
        labeler_id: i32,
        group_id: i32,
        image_id: i32,
    ) -> Result<ApiResponse<ImageDetailResponse>, String> {
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

        // Get the image
        match crate::repository::ImageRepository::find_by_id(db, image_id).await {
            Ok(Some(image)) => {
                // Verify the image belongs to the group
                if image.group_id != group_id {
                    return Err("Image does not belong to this group".to_string());
                }

                // Get group tags
                let group_tags = match crate::repository::TagRepository::get_by_group(db, group_id).await {
                    Ok(tags) => tags.into_iter().map(|tag| TagResponse {
                        id: tag.id,
                        name: tag.name,
                        description: tag.description,
                    }).collect(),
                    Err(e) => {
                        eprintln!("Error fetching group tags: {}", e);
                        return Err(format!("Failed to fetch group tags: {}", e));
                    }
                };

                // Get current tags for this image by this labeler
                let current_tags = match ImageTagsRepository::get_by_image_and_labeler(db, image_id, labeler_id).await {
                    Ok(image_tags) => {
                        // Get the actual tag details
                        let mut tags = Vec::new();
                        for image_tag in image_tags {
                            if let Ok(Some(tag)) = crate::repository::TagRepository::find_by_id(db, image_tag.tag_id).await {
                                tags.push(TagResponse {
                                    id: tag.id,
                                    name: tag.name,
                                    description: tag.description,
                                });
                            }
                        }
                        tags
                    }
                    Err(e) => {
                        eprintln!("Error fetching current tags: {}", e);
                        Vec::new()
                    }
                };

                let image_response = ImageResponse {
                    id: image.id,
                    filename: image.filename,
                    status: if current_tags.is_empty() { "pending".to_string() } else { "done".to_string() },
                    base64_data: image.base64_data,
                    filetype: image.filetype,
                };

                Ok(ApiResponse {
                    success: true,
                    message: "Image details retrieved successfully".to_string(),
                    data: Some(ImageDetailResponse {
                        image: image_response,
                        group_tags,
                        current_tags,
                    }),
                })
            }
            Ok(None) => Err("Image not found".to_string()),
            Err(e) => {
                eprintln!("Error fetching image {}: {}", image_id, e);
                Err(format!("Failed to fetch image: {}", e))
            }
        }
    }

    pub async fn update_image_tags(
        db: &DatabaseConnection,
        labeler_id: i32,
        group_id: i32,
        image_id: i32,
        request: UpdateImageTagsRequest,
    ) -> Result<ApiResponse<()>, String> {
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

        // Verify the image belongs to the group
        match crate::repository::ImageRepository::find_by_id(db, image_id).await {
            Ok(Some(image)) => {
                if image.group_id != group_id {
                    return Err("Image does not belong to this group".to_string());
                }
            }
            Ok(None) => return Err("Image not found".to_string()),
            Err(e) => {
                eprintln!("Error fetching image {}: {}", image_id, e);
                return Err(format!("Failed to fetch image: {}", e));
            }
        }

        // Replace tags for this image by this labeler
        match ImageTagsRepository::replace_tags_for_image_by_labeler(db, image_id, labeler_id, request.tag_ids).await {
            Ok(_) => {
                Ok(ApiResponse {
                    success: true,
                    message: "Image tags updated successfully".to_string(),
                    data: Some(()),
                })
            }
            Err(e) => {
                eprintln!("Error updating image tags: {}", e);
                Err(format!("Failed to update image tags: {}", e))
            }
        }
    }

    pub async fn suggest_tags(
        db: &DatabaseConnection,
        labeler_id: i32,
        image_id: i32,
    ) -> Result<ApiResponse<SuggestTagsResponse>, String> {
        // Get the image
        let image = match crate::repository::ImageRepository::find_by_id(db, image_id).await {
            Ok(Some(image)) => image,
            Ok(None) => return Err("Image not found".to_string()),
            Err(e) => {
                eprintln!("Error fetching image {}: {}", image_id, e);
                return Err(format!("Failed to fetch image: {}", e));
            }
        };

        // Verify the labeler has access to this group
        match LabelerRepository::get_groups(db, labeler_id).await {
            Ok(groups) => {
                if !groups.iter().any(|group| group.id == image.group_id) {
                    return Err("You are not authorized to access this image".to_string());
                }
            }
            Err(e) => {
                eprintln!("Error verifying group access for labeler {}: {}", labeler_id, e);
                return Err(format!("Failed to verify group access: {}", e));
            }
        }

        // Get current tags for this image by this labeler
        let current_tags = match ImageTagsRepository::get_by_image_and_labeler(db, image_id, labeler_id).await {
            Ok(image_tags) => {
                let mut tag_names = Vec::new();
                for image_tag in image_tags {
                    if let Ok(Some(tag)) = crate::repository::TagRepository::find_by_id(db, image_tag.tag_id).await {
                        tag_names.push(tag.name);
                    }
                }
                tag_names
            }
            Err(e) => {
                eprintln!("Error fetching current tags: {}", e);
                Vec::new()
            }
        };

        // Get group tags
        let group_tags: Vec<String> = match crate::repository::TagRepository::get_by_group(db, image.group_id).await {
            Ok(tags) => tags.into_iter().map(|tag| tag.name).collect(),
            Err(e) => {
                eprintln!("Error fetching group tags: {}", e);
                return Err(format!("Failed to fetch group tags: {}", e));
            }
        };

        // Call OpenAI API for suggestions
        match call_openai_for_suggestions(&image.base64_data, &image.filetype, &group_tags, &current_tags).await {
            Ok(suggested_tags) => {
                Ok(ApiResponse {
                    success: true,
                    message: "Tag suggestions generated successfully".to_string(),
                    data: Some(SuggestTagsResponse {
                        suggested_tags,
                    }),
                })
            }
            Err(e) => {
                eprintln!("Error generating tag suggestions: {}", e);
                Err(format!("Failed to generate tag suggestions: {}", e))
            }
        }
    }
}

async fn call_openai_for_suggestions(
    _base64_data: &str,
    _filetype: &str,
    group_tags: &[String],
    current_tags: &[String],
) -> Result<Vec<String>, String> {
    // This is a placeholder implementation
    // In a real implementation, you would call the OpenAI API here
    // For now, we'll return some mock suggestions based on the group tags
    
    let available_tags: Vec<String> = group_tags.iter()
        .filter(|tag| !current_tags.contains(tag))
        .cloned()
        .collect();
    
    // Return up to 3 random suggestions from available tags
    let mut suggestions = available_tags;
    suggestions.truncate(3);
    
    Ok(suggestions)
}
