pub use sea_orm_migration::prelude::*;

mod m20251101_000001_create_api_table;
mod m20251101_000001_create_role_table;
mod m20251101_000001_create_role_api_table;
mod m20251101_000001_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20251101_000001_create_api_table::Migration),
             Box::new(m20251101_000001_create_role_table::Migration),
             Box::new(m20251101_000001_create_role_api_table::Migration),
             Box::new(m20251101_000001_create_user_table::Migration),
        ]
    }
}
