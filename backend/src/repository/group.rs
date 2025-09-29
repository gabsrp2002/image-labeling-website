use sea_orm::*;
use crate::entity::group::{Entity as Group, Model as GroupModel, ActiveModel as GroupActiveModel};
use crate::entity::image::{Entity as Image, Model as ImageModel};
use crate::entity::tag::{Entity as Tag, Model as TagModel};
use crate::entity::labeler::{Entity as Labeler, Model as LabelerModel};

pub struct GroupRepository;

impl GroupRepository {
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        description: Option<String>,
    ) -> Result<GroupModel, DbErr> {
        let group = GroupActiveModel {
            name: Set(name),
            description: Set(description),
            ..Default::default()
        };
        
        group.insert(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<GroupModel>, DbErr> {
        Group::find_by_id(id).one(db).await
    }

    pub async fn get_all(
        db: &DatabaseConnection,
    ) -> Result<Vec<GroupModel>, DbErr> {
        Group::find().all(db).await
    }

    pub async fn get_images(
        db: &DatabaseConnection,
        group_id: i32,
    ) -> Result<Vec<ImageModel>, DbErr> {
        Image::find()
            .filter(crate::entity::image::Column::GroupId.eq(group_id))
            .all(db)
            .await
    }

    pub async fn get_possible_tags(
        db: &DatabaseConnection,
        group_id: i32,
    ) -> Result<Vec<TagModel>, DbErr> {
        Tag::find()
            .filter(crate::entity::tag::Column::GroupId.eq(group_id))
            .all(db)
            .await
    }

    pub async fn get_labelers(
        db: &DatabaseConnection,
        group_id: i32,
    ) -> Result<Vec<LabelerModel>, DbErr> {
        use crate::entity::labeler_groups::Entity as LabelerGroups;
        use crate::entity::labeler::Entity as Labeler;
        
        let labelers = Labeler::find()
            .inner_join(LabelerGroups)
            .filter(crate::entity::labeler_groups::Column::GroupId.eq(group_id))
            .all(db)
            .await?;
        
        Ok(labelers)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<GroupModel, DbErr> {
        let group = Group::find_by_id(id).one(db).await?;
        match group {
            Some(group) => {
                let mut group: GroupActiveModel = group.into();
                if let Some(name) = name {
                    group.name = Set(name);
                }
                if let Some(description) = description {
                    group.description = Set(Some(description));
                }
                group.update(db).await
            }
            None => Err(DbErr::RecordNotFound("Group not found".to_string())),
        }
    }

    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<(), DbErr> {
        Group::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
