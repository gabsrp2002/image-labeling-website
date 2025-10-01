use sea_orm::DatabaseConnection;
use crate::repository::{GroupRepository, ImageRepository, FinalTagsRepository, TagRepository};
use crate::schemas::export::{ExportData, GroupData, ImageData};

pub struct ExportService;

impl ExportService {
    pub async fn bulk_export(db: &DatabaseConnection) -> Result<ExportData, String> {
        let groups = GroupRepository::get_all(db).await
            .map_err(|_| "Failed to retrieve groups".to_string())?;
        
        let mut export_data = std::collections::HashMap::new();
        
        for group in groups {
            let group_id = group.id.to_string();
            let mut group_data = std::collections::HashMap::new();
            
            // Get all images for this group
            let images = ImageRepository::get_by_group(db, group.id).await
                .map_err(|_| format!("Failed to retrieve images for group {}", group.id))?;
            
            for image in images {
                let image_id = image.id.to_string();
                
                // Get final tags for this image
                let final_tags = Self::get_final_tags_for_image(db, image.id).await?;
                
                group_data.insert(image_id, ImageData {
                    filename: image.filename,
                    filetype: image.filetype,
                    base64: image.base64_data,
                    final_tags,
                });
            }
            
            export_data.insert(group_id, GroupData {
                images: group_data,
            });
        }
        
        Ok(ExportData {
            groups: export_data,
        })
    }
    
    async fn get_final_tags_for_image(db: &DatabaseConnection, image_id: i32) -> Result<Vec<String>, String> {
        let final_tags = FinalTagsRepository::get_by_image(db, image_id).await
            .map_err(|_| format!("Failed to retrieve final tags for image {}", image_id))?;
        
        let mut tag_names = Vec::new();
        for tag in final_tags {
            match TagRepository::find_by_id(db, tag.tag_id).await {
                Ok(Some(tag_model)) => {
                    tag_names.push(tag_model.name);
                }
                Ok(None) => continue, // Tag not found, skip
                Err(_) => continue, // Error getting tag, skip
            }
        }
        
        Ok(tag_names)
    }
}
