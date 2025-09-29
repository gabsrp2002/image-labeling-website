use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::image::Entity")]
    Images,
    #[sea_orm(has_many = "super::tag::Entity")]
    PossibleTags,
    #[sea_orm(has_many = "super::labeler_groups::Entity")]
    LabelerGroups,
}

impl Related<super::image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Images.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PossibleTags.def()
    }
}

impl Related<super::labeler::Entity> for Entity {
    fn to() -> RelationDef {
        super::labeler_groups::Relation::Labeler.def()
    }
}

impl Related<super::labeler_groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabelerGroups.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
