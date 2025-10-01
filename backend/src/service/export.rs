use sea_orm::DatabaseConnection;
use crate::repository::{GroupRepository, ImageRepository, FinalTagsRepository, TagRepository, ImageTagsRepository};
use crate::schemas::export::{ExportData, GroupData, ImageData, TagStatistic};

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
                
                // Get tag statistics for this image
                let tag_statistics = Self::get_tag_statistics_for_image(db, group.id, image.id).await?;
                
                // Check if there's an admin override
                let has_admin_override = FinalTagsRepository::has_admin_override(db, image.id).await
                    .map_err(|_| format!("Failed to check admin override for image {}", image.id))?;
                
                group_data.insert(image_id, ImageData {
                    filename: image.filename,
                    filetype: image.filetype,
                    base64: image.base64_data,
                    uploaded_at: image.uploaded_at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                    final_tags,
                    tag_statistics,
                    has_admin_override,
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
    
    async fn get_tag_statistics_for_image(db: &DatabaseConnection, group_id: i32, image_id: i32) -> Result<Vec<TagStatistic>, String> {
        // Get all image tags for this image
        let image_tags = ImageTagsRepository::get_all_tags_for_image(db, image_id).await
            .map_err(|_| format!("Failed to retrieve image tags for image {}", image_id))?;
        
        // Calculate tag counts
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
        let all_group_tags = GroupRepository::get_possible_tags(db, group_id).await
            .map_err(|_| format!("Failed to retrieve possible tags for group {}", group_id))?;
        
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
        
        Ok(tag_statistics)
    }
}
