use sea_orm_migration::prelude::*;

mod m20261028_000001_create_notes;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20261028_000001_create_notes::Migration)]
    }
}
