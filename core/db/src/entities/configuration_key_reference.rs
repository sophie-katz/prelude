//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "configuration_key_reference")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub description: String,
    pub type_id: i32,
    pub optional: bool,
    pub allows_multiple: bool,
    pub allows_user_override: bool,
    pub deactivate_timestamp: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::configuration_entries::Entity")]
    ConfigurationEntries,
    #[sea_orm(
        belongs_to = "super::configuration_type_reference::Entity",
        from = "Column::TypeId",
        to = "super::configuration_type_reference::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ConfigurationTypeReference,
}

impl Related<super::configuration_entries::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConfigurationEntries.def()
    }
}

impl Related<super::configuration_type_reference::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConfigurationTypeReference.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
