use sea_orm::*;
use crate::entity::tag::{Entity as Tag, Model as TagModel, ActiveModel as TagActiveModel};

pub struct TagRepository;

impl TagRepository {
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        description: Option<String>,
        group_id: i32,
    ) -> Result<TagModel, DbErr> {
        let tag = TagActiveModel {
            name: Set(name),
            description: Set(description),
            group_id: Set(group_id),
            ..Default::default()
        };
        
        tag.insert(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<TagModel>, DbErr> {
        Tag::find_by_id(id).one(db).await
    }

    pub async fn get_by_group(
        db: &DatabaseConnection,
        group_id: i32,
    ) -> Result<Vec<TagModel>, DbErr> {
        Tag::find()
            .filter(crate::entity::tag::Column::GroupId.eq(group_id))
            .all(db)
            .await
    }

    pub async fn find_by_name_and_group(
        db: &DatabaseConnection,
        name: &str,
        group_id: i32,
    ) -> Result<Option<TagModel>, DbErr> {
        Tag::find()
            .filter(crate::entity::tag::Column::Name.eq(name))
            .filter(crate::entity::tag::Column::GroupId.eq(group_id))
            .one(db)
            .await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<TagModel, DbErr> {
        let tag = Tag::find_by_id(id).one(db).await?;
        match tag {
            Some(tag) => {
                let mut tag: TagActiveModel = tag.into();
                if let Some(name) = name {
                    tag.name = Set(name);
                }
                if let Some(description) = description {
                    tag.description = Set(Some(description));
                }
                tag.update(db).await
            }
            None => Err(DbErr::RecordNotFound("Tag not found".to_string())),
        }
    }

    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<(), DbErr> {
        Tag::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
