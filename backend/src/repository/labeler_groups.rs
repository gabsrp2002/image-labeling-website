use sea_orm::*;
use crate::entity::labeler_groups::{Entity as LabelerGroups, Model as LabelerGroupsModel, ActiveModel as LabelerGroupsActiveModel};

pub struct LabelerGroupsRepository;

impl LabelerGroupsRepository {
    pub async fn create(
        db: &DatabaseConnection,
        labeler_id: i32,
        group_id: i32,
    ) -> Result<LabelerGroupsModel, DbErr> {
        let labeler_group = LabelerGroupsActiveModel {
            labeler_id: Set(labeler_id),
            group_id: Set(group_id),
            ..Default::default()
        };
        
        labeler_group.insert(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<LabelerGroupsModel>, DbErr> {
        LabelerGroups::find_by_id(id).one(db).await
    }

    pub async fn find_by_labeler_and_group(
        db: &DatabaseConnection,
        labeler_id: i32,
        group_id: i32,
    ) -> Result<Option<LabelerGroupsModel>, DbErr> {
        LabelerGroups::find()
            .filter(crate::entity::labeler_groups::Column::LabelerId.eq(labeler_id))
            .filter(crate::entity::labeler_groups::Column::GroupId.eq(group_id))
            .one(db)
            .await
    }

    pub async fn get_by_labeler(
        db: &DatabaseConnection,
        labeler_id: i32,
    ) -> Result<Vec<LabelerGroupsModel>, DbErr> {
        LabelerGroups::find()
            .filter(crate::entity::labeler_groups::Column::LabelerId.eq(labeler_id))
            .all(db)
            .await
    }

    pub async fn get_by_group(
        db: &DatabaseConnection,
        group_id: i32,
    ) -> Result<Vec<LabelerGroupsModel>, DbErr> {
        LabelerGroups::find()
            .filter(crate::entity::labeler_groups::Column::GroupId.eq(group_id))
            .all(db)
            .await
    }

    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<(), DbErr> {
        LabelerGroups::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn delete_by_labeler_and_group(
        db: &DatabaseConnection,
        labeler_id: i32,
        group_id: i32,
    ) -> Result<(), DbErr> {
        LabelerGroups::delete_many()
            .filter(crate::entity::labeler_groups::Column::LabelerId.eq(labeler_id))
            .filter(crate::entity::labeler_groups::Column::GroupId.eq(group_id))
            .exec(db)
            .await?;
        
        Ok(())
    }
}
