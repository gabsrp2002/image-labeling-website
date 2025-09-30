use sea_orm::*;
use crate::entity::image::{Entity as Image, Model as ImageModel, ActiveModel as ImageActiveModel};
use crate::entity::image_tags::{Entity as ImageTags, Model as ImageTagsModel};

pub struct ImageRepository;

impl ImageRepository {
    pub async fn create(
        db: &DatabaseConnection,
        filename: String,
        filetype: String,
        base64_data: String,
        group_id: i32,
    ) -> Result<ImageModel, DbErr> {
        let image = ImageActiveModel {
            filename: Set(filename),
            filetype: Set(filetype),
            base64_data: Set(base64_data),
            group_id: Set(group_id),
            uploaded_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        
        image.insert(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<ImageModel>, DbErr> {
        Image::find_by_id(id).one(db).await
    }

    pub async fn get_by_group(
        db: &DatabaseConnection,
        group_id: i32,
    ) -> Result<Vec<ImageModel>, DbErr> {
        Image::find()
            .filter(crate::entity::image::Column::GroupId.eq(group_id))
            .all(db)
            .await
    }

    pub async fn get_tags(
        db: &DatabaseConnection,
        image_id: i32,
    ) -> Result<Vec<ImageTagsModel>, DbErr> {
        ImageTags::find()
            .filter(crate::entity::image_tags::Column::ImageId.eq(image_id))
            .all(db)
            .await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        filename: Option<String>,
        filetype: Option<String>,
        base64_data: Option<String>,
        group_id: Option<i32>,
    ) -> Result<ImageModel, DbErr> {
        let image = Image::find_by_id(id).one(db).await?;
        match image {
            Some(image) => {
                let mut image: ImageActiveModel = image.into();
                if let Some(filename) = filename {
                    image.filename = Set(filename);
                }
                if let Some(filetype) = filetype {
                    image.filetype = Set(filetype);
                }
                if let Some(base64_data) = base64_data {
                    image.base64_data = Set(base64_data);
                }
                if let Some(group_id) = group_id {
                    image.group_id = Set(group_id);
                }
                image.update(db).await
            }
            None => Err(DbErr::RecordNotFound("Image not found".to_string())),
        }
    }

    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<(), DbErr> {
        Image::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
