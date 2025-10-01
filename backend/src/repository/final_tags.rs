use sea_orm::*;
use crate::entity::final_tags::{Entity as FinalTags, Model as FinalTagsModel, ActiveModel as FinalTagsActiveModel};

pub struct FinalTagsRepository;

impl FinalTagsRepository {
    pub async fn create(
        db: &DatabaseConnection,
        image_id: i32,
        tag_id: i32,
        is_admin_override: bool,
    ) -> Result<FinalTagsModel, DbErr> {
        let final_tag = FinalTagsActiveModel {
            image_id: Set(image_id),
            tag_id: Set(tag_id),
            is_admin_override: Set(is_admin_override),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        
        final_tag.insert(db).await
    }

    pub async fn get_by_image(
        db: &DatabaseConnection,
        image_id: i32,
    ) -> Result<Vec<FinalTagsModel>, DbErr> {
        FinalTags::find()
            .filter(crate::entity::final_tags::Column::ImageId.eq(image_id))
            .all(db)
            .await
    }

    pub async fn delete_by_image(
        db: &DatabaseConnection,
        image_id: i32,
    ) -> Result<(), DbErr> {
        FinalTags::delete_many()
            .filter(crate::entity::final_tags::Column::ImageId.eq(image_id))
            .exec(db)
            .await?;
        
        Ok(())
    }

    pub async fn replace_final_tags(
        db: &DatabaseConnection,
        image_id: i32,
        tag_ids: Vec<i32>,
        is_admin_override: bool,
    ) -> Result<Vec<FinalTagsModel>, DbErr> {
        // First, remove all existing final tags for this image
        Self::delete_by_image(db, image_id).await?;
        
        // Then create new final tags
        let mut results = Vec::new();
        for tag_id in tag_ids {
            let final_tag = Self::create(db, image_id, tag_id, is_admin_override).await?;
            results.push(final_tag);
        }
        
        Ok(results)
    }

    pub async fn has_admin_override(
        db: &DatabaseConnection,
        image_id: i32,
    ) -> Result<bool, DbErr> {
        let final_tags = FinalTags::find()
            .filter(crate::entity::final_tags::Column::ImageId.eq(image_id))
            .filter(crate::entity::final_tags::Column::IsAdminOverride.eq(true))
            .one(db)
            .await?;
        
        Ok(final_tags.is_some())
    }
}
