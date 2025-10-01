use sea_orm::*;
use crate::entity::image_tags::{Entity as ImageTags, Model as ImageTagsModel, ActiveModel as ImageTagsActiveModel};

pub struct ImageTagsRepository;

impl ImageTagsRepository {
    pub async fn create(
        db: &DatabaseConnection,
        image_id: i32,
        labeler_id: i32,
        tag_id: i32,
    ) -> Result<ImageTagsModel, DbErr> {
        let image_tag = ImageTagsActiveModel {
            image_id: Set(image_id),
            labeler_id: Set(labeler_id),
            tag_id: Set(tag_id),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        
        image_tag.insert(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<ImageTagsModel>, DbErr> {
        ImageTags::find_by_id(id).one(db).await
    }

    pub async fn get_by_image(
        db: &DatabaseConnection,
        image_id: i32,
    ) -> Result<Vec<ImageTagsModel>, DbErr> {
        ImageTags::find()
            .filter(crate::entity::image_tags::Column::ImageId.eq(image_id))
            .all(db)
            .await
    }

    pub async fn get_by_labeler(
        db: &DatabaseConnection,
        labeler_id: i32,
    ) -> Result<Vec<ImageTagsModel>, DbErr> {
        ImageTags::find()
            .filter(crate::entity::image_tags::Column::LabelerId.eq(labeler_id))
            .all(db)
            .await
    }

    pub async fn get_by_image_and_labeler(
        db: &DatabaseConnection,
        image_id: i32,
        labeler_id: i32,
    ) -> Result<Vec<ImageTagsModel>, DbErr> {
        ImageTags::find()
            .filter(crate::entity::image_tags::Column::ImageId.eq(image_id))
            .filter(crate::entity::image_tags::Column::LabelerId.eq(labeler_id))
            .all(db)
            .await
    }

    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<(), DbErr> {
        ImageTags::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn delete_by_image_and_labeler_and_tag(
        db: &DatabaseConnection,
        image_id: i32,
        labeler_id: i32,
        tag_id: i32,
    ) -> Result<(), DbErr> {
        ImageTags::delete_many()
            .filter(crate::entity::image_tags::Column::ImageId.eq(image_id))
            .filter(crate::entity::image_tags::Column::LabelerId.eq(labeler_id))
            .filter(crate::entity::image_tags::Column::TagId.eq(tag_id))
            .exec(db)
            .await?;
        
        Ok(())
    }

    pub async fn get_tags_for_image_by_labeler(
        db: &DatabaseConnection,
        image_id: i32,
        labeler_id: i32,
    ) -> Result<Vec<ImageTagsModel>, DbErr> {
        ImageTags::find()
            .filter(crate::entity::image_tags::Column::ImageId.eq(image_id))
            .filter(crate::entity::image_tags::Column::LabelerId.eq(labeler_id))
            .all(db)
            .await
    }

    pub async fn assign_multiple_tags(
        db: &DatabaseConnection,
        image_id: i32,
        labeler_id: i32,
        tag_ids: Vec<i32>,
    ) -> Result<Vec<ImageTagsModel>, DbErr> {
        let mut results = Vec::new();
        
        for tag_id in tag_ids {
            // Check if this tag assignment already exists
            let existing = ImageTags::find()
                .filter(crate::entity::image_tags::Column::ImageId.eq(image_id))
                .filter(crate::entity::image_tags::Column::LabelerId.eq(labeler_id))
                .filter(crate::entity::image_tags::Column::TagId.eq(tag_id))
                .one(db)
                .await?;
            
            if existing.is_none() {
                let image_tag = Self::create(db, image_id, labeler_id, tag_id).await?;
                results.push(image_tag);
            }
        }
        
        Ok(results)
    }

    pub async fn replace_tags_for_image_by_labeler(
        db: &DatabaseConnection,
        image_id: i32,
        labeler_id: i32,
        tag_ids: Vec<i32>,
    ) -> Result<Vec<ImageTagsModel>, DbErr> {
        // First, remove all existing tags for this image-labeler combination
        ImageTags::delete_many()
            .filter(crate::entity::image_tags::Column::ImageId.eq(image_id))
            .filter(crate::entity::image_tags::Column::LabelerId.eq(labeler_id))
            .exec(db)
            .await?;
        
        // Then assign the new tags
        Self::assign_multiple_tags(db, image_id, labeler_id, tag_ids).await
    }

    pub async fn get_all_tags_for_image(
        db: &DatabaseConnection,
        image_id: i32,
    ) -> Result<Vec<ImageTagsModel>, DbErr> {
        ImageTags::find()
            .filter(crate::entity::image_tags::Column::ImageId.eq(image_id))
            .all(db)
            .await
    }

    pub async fn delete_by_labeler_and_group(
        db: &DatabaseConnection,
        labeler_id: i32,
        group_id: i32,
    ) -> Result<(), DbErr> {
        use crate::entity::image::Entity as Image;
        
        // First get all images in the group
        let images = Image::find()
            .filter(crate::entity::image::Column::GroupId.eq(group_id))
            .all(db)
            .await?;
        
        // Extract image IDs
        let image_ids: Vec<i32> = images.into_iter().map(|img| img.id).collect();
        
        if !image_ids.is_empty() {
            // Delete all image tags for this labeler from images in this group
            ImageTags::delete_many()
                .filter(crate::entity::image_tags::Column::LabelerId.eq(labeler_id))
                .filter(crate::entity::image_tags::Column::ImageId.is_in(image_ids))
                .exec(db)
                .await?;
        }
        
        Ok(())
    }
}
