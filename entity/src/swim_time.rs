//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.11

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "swim_time")]
#[graphql(name = "SwimTimeResult")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub competition: i32,
    pub distance: i32,
    pub stroke: String,
    pub time: i32,
    pub swimmer: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::competition::Entity",
        from = "Column::Competition",
        to = "super::competition::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Competition,
    #[sea_orm(
        belongs_to = "super::swimmer::Entity",
        from = "Column::Swimmer",
        to = "super::swimmer::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Swimmer,
}

impl Related<super::competition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Competition.def()
    }
}

impl Related<super::swimmer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Swimmer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
