use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "labeler_groups")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub labeler_id: i32,
    pub group_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::labeler::Entity",
        from = "Column::LabelerId",
        to = "super::labeler::Column::Id"
    )]
    Labeler,
    #[sea_orm(
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::Id"
    )]
    Group,
}

impl Related<super::labeler::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Labeler.def()
    }
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
