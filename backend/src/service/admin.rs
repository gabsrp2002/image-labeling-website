use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, ColumnTrait, QueryFilter};
use bcrypt::hash;
use crate::repository::{LabelerRepository, GroupRepository};
use crate::schemas::admin::{
    CreateLabelerRequest, UpdateLabelerRequest, LabelerResponse, 
    LabelerListResponse, GroupResponse, GroupListResponse, ApiResponse,
    CreateGroupRequest
};

pub struct AdminService;

impl AdminService {
    pub async fn create_labeler(
        db: &DatabaseConnection,
        request: CreateLabelerRequest,
    ) -> Result<ApiResponse<LabelerResponse>, String> {
        // Check if labeler already exists
        match LabelerRepository::find_by_username(db, &request.username).await {
            Ok(Some(_)) => {
                return Ok(ApiResponse {
                    success: false,
                    message: "Labeler with this username already exists".to_string(),
                    data: None,
                });
            }
            Ok(None) => {} // Continue with creation
            Err(e) => return Err(format!("Database error: {}", e)),
        }

        // Hash the password
        let password_hash = hash(&request.password, bcrypt::DEFAULT_COST)
            .map_err(|e| format!("Password hashing failed: {}", e))?;

        // Create the labeler
        match LabelerRepository::create(db, request.username.clone(), password_hash).await {
            Ok(labeler) => {
                // Assign groups if provided
                if let Some(group_ids) = &request.group_ids {
                    for &group_id in group_ids {
                        if let Err(e) = LabelerRepository::add_to_group(db, labeler.id, group_id).await {
                            eprintln!("Warning: Failed to add labeler {} to group {}: {}", labeler.id, group_id, e);
                        }
                    }
                }

                // Get the labeler's groups
                let labeler_groups = LabelerRepository::get_groups(db, labeler.id).await.unwrap_or_default();
                let group_ids = labeler_groups.into_iter().map(|g| g.id).collect();

                let response = LabelerResponse {
                    id: labeler.id,
                    username: labeler.username,
                    group_ids,
                };
                Ok(ApiResponse {
                    success: true,
                    message: "Labeler created successfully".to_string(),
                    data: Some(response),
                })
            }
            Err(e) => Err(format!("Failed to create labeler: {}", e)),
        }
    }

    pub async fn get_labeler(
        db: &DatabaseConnection,
        labeler_id: i32,
    ) -> Result<ApiResponse<LabelerResponse>, String> {
        match LabelerRepository::find_by_id(db, labeler_id).await {
            Ok(Some(labeler)) => {
                // Get the labeler's groups
                let labeler_groups = LabelerRepository::get_groups(db, labeler.id).await.unwrap_or_default();
                let group_ids = labeler_groups.into_iter().map(|g| g.id).collect();

                let response = LabelerResponse {
                    id: labeler.id,
                    username: labeler.username,
                    group_ids,
                };
                Ok(ApiResponse {
                    success: true,
                    message: "Labeler retrieved successfully".to_string(),
                    data: Some(response),
                })
            }
            Ok(None) => {
                Ok(ApiResponse {
                    success: false,
                    message: "Labeler not found".to_string(),
                    data: None,
                })
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn list_labelers(
        db: &DatabaseConnection,
    ) -> Result<ApiResponse<LabelerListResponse>, String> {
        use crate::entity::labeler::Entity as Labeler;
        
        match Labeler::find().all(db).await {
            Ok(labelers) => {
                let mut labeler_responses = Vec::new();
                
                for labeler in labelers {
                    // Get the labeler's groups
                    let labeler_groups = LabelerRepository::get_groups(db, labeler.id).await.unwrap_or_default();
                    let group_ids = labeler_groups.into_iter().map(|g| g.id).collect();
                    
                    labeler_responses.push(LabelerResponse {
                        id: labeler.id,
                        username: labeler.username,
                        group_ids,
                    });
                }

                let response = LabelerListResponse {
                    total: labeler_responses.len(),
                    labelers: labeler_responses,
                };

                Ok(ApiResponse {
                    success: true,
                    message: "Labelers retrieved successfully".to_string(),
                    data: Some(response),
                })
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn update_labeler(
        db: &DatabaseConnection,
        labeler_id: i32,
        request: UpdateLabelerRequest,
    ) -> Result<ApiResponse<LabelerResponse>, String> {
        // First, get the existing labeler
        let labeler = match LabelerRepository::find_by_id(db, labeler_id).await {
            Ok(Some(labeler)) => labeler,
            Ok(None) => {
                return Ok(ApiResponse {
                    success: false,
                    message: "Labeler not found".to_string(),
                    data: None,
                });
            }
            Err(e) => return Err(format!("Database error: {}", e)),
        };

        // Check if username is being changed and if it already exists
        if let Some(new_username) = &request.username {
            if new_username != &labeler.username {
                match LabelerRepository::find_by_username(db, new_username).await {
                    Ok(Some(_)) => {
                        return Ok(ApiResponse {
                            success: false,
                            message: "Username already exists".to_string(),
                            data: None,
                        });
                    }
                    Ok(None) => {} // Username is available
                    Err(e) => return Err(format!("Database error: {}", e)),
                }
            }
        }

        // Update the labeler
        use crate::entity::labeler::ActiveModel as LabelerActiveModel;
        use sea_orm::Set;

        let mut labeler_active: LabelerActiveModel = labeler.into();

        if let Some(username) = request.username {
            labeler_active.username = Set(username);
        }

        if let Some(password) = request.password {
            let password_hash = hash(&password, bcrypt::DEFAULT_COST)
                .map_err(|e| format!("Password hashing failed: {}", e))?;
            labeler_active.password_hash = Set(password_hash);
        }

        match labeler_active.update(db).await {
            Ok(updated_labeler) => {
                // Update group assignments if provided
                if let Some(group_ids) = &request.group_ids {
                    // Remove all existing group assignments
                    if let Err(e) = LabelerRepository::remove_from_all_groups(db, updated_labeler.id).await {
                        eprintln!("Warning: Failed to remove labeler {} from all groups: {}", updated_labeler.id, e);
                    }
                    
                    // Add new group assignments
                    for &group_id in group_ids {
                        if let Err(e) = LabelerRepository::add_to_group(db, updated_labeler.id, group_id).await {
                            eprintln!("Warning: Failed to add labeler {} to group {}: {}", updated_labeler.id, group_id, e);
                        }
                    }
                }

                // Get the labeler's groups
                let labeler_groups = LabelerRepository::get_groups(db, updated_labeler.id).await.unwrap_or_default();
                let group_ids = labeler_groups.into_iter().map(|g| g.id).collect();

                let response = LabelerResponse {
                    id: updated_labeler.id,
                    username: updated_labeler.username,
                    group_ids,
                };
                Ok(ApiResponse {
                    success: true,
                    message: "Labeler updated successfully".to_string(),
                    data: Some(response),
                })
            }
            Err(e) => Err(format!("Failed to update labeler: {}", e)),
        }
    }

    pub async fn delete_labeler(
        db: &DatabaseConnection,
        labeler_id: i32,
    ) -> Result<ApiResponse<()>, String> {
        // Check if labeler exists
        match LabelerRepository::find_by_id(db, labeler_id).await {
            Ok(Some(_)) => {} // Labeler exists, continue with deletion
            Ok(None) => {
                return Ok(ApiResponse {
                    success: false,
                    message: "Labeler not found".to_string(),
                    data: None,
                });
            }
            Err(e) => return Err(format!("Database error: {}", e)),
        }

        // Delete the labeler
        use crate::entity::labeler::{Entity as Labeler, Column};
        
        match Labeler::delete_many()
            .filter(Column::Id.eq(labeler_id))
            .exec(db)
            .await {
            Ok(_) => {
                Ok(ApiResponse {
                    success: true,
                    message: "Labeler deleted successfully".to_string(),
                    data: Some(()),
                })
            }
            Err(e) => Err(format!("Failed to delete labeler: {}", e)),
        }
    }

    pub async fn create_group(
        db: &DatabaseConnection,
        request: CreateGroupRequest,
    ) -> Result<ApiResponse<GroupResponse>, String> {
        match GroupRepository::create(db, request.name, request.description).await {
            Ok(group) => {
                let response = GroupResponse {
                    id: group.id,
                    name: group.name,
                    description: group.description,
                };
                Ok(ApiResponse {
                    success: true,
                    message: "Group created successfully".to_string(),
                    data: Some(response),
                })
            }
            Err(e) => Err(format!("Failed to create group: {}", e)),
        }
    }

    pub async fn list_groups(
        db: &DatabaseConnection,
    ) -> Result<ApiResponse<GroupListResponse>, String> {
        match GroupRepository::get_all(db).await {
            Ok(groups) => {
                let group_responses: Vec<GroupResponse> = groups
                    .into_iter()
                    .map(|group| GroupResponse {
                        id: group.id,
                        name: group.name,
                        description: group.description,
                    })
                    .collect();

                let response = GroupListResponse {
                    total: group_responses.len(),
                    groups: group_responses,
                };

                Ok(ApiResponse {
                    success: true,
                    message: "Groups retrieved successfully".to_string(),
                    data: Some(response),
                })
            }
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn delete_group(
        db: &DatabaseConnection,
        group_id: i32,
    ) -> Result<ApiResponse<()>, String> {
        // Check if group exists
        match GroupRepository::find_by_id(db, group_id).await {
            Ok(Some(_)) => {} // Group exists, continue with deletion
            Ok(None) => {
                return Ok(ApiResponse {
                    success: false,
                    message: "Group not found".to_string(),
                    data: None,
                });
            }
            Err(e) => return Err(format!("Database error: {}", e)),
        }

        // Delete the group
        match GroupRepository::delete(db, group_id).await {
            Ok(_) => {
                Ok(ApiResponse {
                    success: true,
                    message: "Group deleted successfully".to_string(),
                    data: Some(()),
                })
            }
            Err(e) => Err(format!("Failed to delete group: {}", e)),
        }
    }
}
