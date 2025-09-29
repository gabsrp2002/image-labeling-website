use sea_orm::*;
use crate::entity::image::{Entity as Image, Model as ImageModel, ActiveModel as ImageActiveModel};
use crate::entity::image_tags::{Entity as ImageTags, Model as ImageTagsModel};

pub struct ImageRepository;

impl ImageRepository {
    pub async fn create(
        db: &DatabaseConnection,
        path: String,
        name: String,
        group_id: i32,
    ) -> Result<ImageModel, DbErr> {
        let image = ImageActiveModel {
            path: Set(path),
            name: Set(name),
            group_id: Set(group_id),
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
        path: Option<String>,
        name: Option<String>,
        group_id: Option<i32>,
    ) -> Result<ImageModel, DbErr> {
        let image = Image::find_by_id(id).one(db).await?;
        match image {
            Some(image) => {
                let mut image: ImageActiveModel = image.into();
                if let Some(path) = path {
                    image.path = Set(path);
                }
                if let Some(name) = name {
                    image.name = Set(name);
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
