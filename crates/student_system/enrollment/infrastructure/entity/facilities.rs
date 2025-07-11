//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.13

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "facilities")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub capacity: Option<i32>,
    pub facility_type: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::course_schedules::Entity")]
    CourseSchedules,
}

impl Related<super::course_schedules::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CourseSchedules.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
