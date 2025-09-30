use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "image")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub filename: String,
    pub filetype: String,
    pub base64_data: String,
    pub group_id: i32,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::Id"
    )]
    Group,
    #[sea_orm(has_many = "super::image_tags::Entity")]
    ImageTags,
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<super::image_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ImageTags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
