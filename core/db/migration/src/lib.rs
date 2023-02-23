use sea_orm_migration::prelude::*;

mod m20230218_120854_create_configuration_type_reference_table;
mod m20230218_120923_create_configuration_key_reference_table;
mod m20230219_142203_create_configuration_entries_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230218_120854_create_configuration_type_reference_table::Migration),
            Box::new(m20230218_120923_create_configuration_key_reference_table::Migration),
            Box::new(m20230219_142203_create_configuration_entries_table::Migration),
        ]
    }
}
