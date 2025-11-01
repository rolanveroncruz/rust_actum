// m20251101_000001_create_api_table.rs
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migrationxxxx scripts

        manager
            .create_table(
                Table::create()
                    .table(API::Table)
                    .if_not_exists()
                    .col(pk_auto(API::Id))
                    .col(string(API::Path))
                    .col(string(API::Method))
                    .col(string(API::Desc))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migrationxxxx scripts

        manager
            .drop_table(Table::drop().table(API::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum API{
    Table,
    Id,
    Path,
    Method,
    Desc,
}
