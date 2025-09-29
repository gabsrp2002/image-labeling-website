use sea_orm::*;
use crate::entity::labeler::{Entity as Labeler, Model as LabelerModel, ActiveModel as LabelerActiveModel};
use crate::entity::group::Model as GroupModel;

pub struct LabelerRepository;

impl LabelerRepository {
    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<LabelerModel>, DbErr> {
        Labeler::find()
            .filter(crate::entity::labeler::Column::Username.eq(username))
            .one(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        username: String,
        password_hash: String,
    ) -> Result<LabelerModel, DbErr> {
        let labeler = LabelerActiveModel {
            username: Set(username),
            password_hash: Set(password_hash),
            ..Default::default()
        };
        
        labeler.insert(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<LabelerModel>, DbErr> {
        Labeler::find_by_id(id).one(db).await
    }

    pub async fn get_groups(
        db: &DatabaseConnection,
        labeler_id: i32,
    ) -> Result<Vec<GroupModel>, DbErr> {
        use crate::entity::labeler_groups::Entity as LabelerGroups;
        use crate::entity::group::Entity as Group;
        
        let groups = Group::find()
            .inner_join(LabelerGroups)
            .filter(crate::entity::labeler_groups::Column::LabelerId.eq(labeler_id))
            .all(db)
            .await?;
        
        Ok(groups)
    }

    pub async fn add_to_group(
        db: &DatabaseConnection,
        labeler_id: i32,
        group_id: i32,
    ) -> Result<(), DbErr> {
        use crate::entity::labeler_groups::ActiveModel as LabelerGroupsActiveModel;
        
        let labeler_group = LabelerGroupsActiveModel {
            labeler_id: Set(labeler_id),
            group_id: Set(group_id),
            ..Default::default()
        };
        
        labeler_group.insert(db).await?;
        Ok(())
    }

    pub async fn remove_from_group(
        db: &DatabaseConnection,
        labeler_id: i32,
        group_id: i32,
    ) -> Result<(), DbErr> {
        use crate::entity::labeler_groups::Entity as LabelerGroups;
        
        LabelerGroups::delete_many()
            .filter(crate::entity::labeler_groups::Column::LabelerId.eq(labeler_id))
            .filter(crate::entity::labeler_groups::Column::GroupId.eq(group_id))
            .exec(db)
            .await?;
        
        Ok(())
    }
}
