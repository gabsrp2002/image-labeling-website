use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "labeler")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::labeler_groups::Entity")]
    LabelerGroups,
    #[sea_orm(has_many = "super::image_tags::Entity")]
    ImageTags,
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        super::labeler_groups::Relation::Group.def()
    }
}

impl Related<super::labeler_groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabelerGroups.def()
    }
}

impl Related<super::image_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ImageTags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
