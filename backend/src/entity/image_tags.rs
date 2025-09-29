use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "image_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub image_id: i32,
    pub labeler_id: i32,
    pub tag_id: i32,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::image::Entity",
        from = "Column::ImageId",
        to = "super::image::Column::Id"
    )]
    Image,
    #[sea_orm(
        belongs_to = "super::labeler::Entity",
        from = "Column::LabelerId",
        to = "super::labeler::Column::Id"
    )]
    Labeler,
    #[sea_orm(
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id"
    )]
    Tag,
}

impl Related<super::image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Image.def()
    }
}

impl Related<super::labeler::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Labeler.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
