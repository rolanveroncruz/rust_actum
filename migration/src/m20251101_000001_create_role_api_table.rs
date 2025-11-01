// m20251101_000001_create_role_api_table.rs
use sea_orm_migration::{prelude::*, schema::*};
use crate::m20251101_000001_create_api_table::API;
use crate::m20251101_000001_create_role_table::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migrationxxxx scripts

        manager
            .create_table(
                Table::create()
                    .table(RoleAPI::Table)
                    .if_not_exists()
                    .col(pk_auto(RoleAPI::Id))
                    .col(ColumnDef::new(RoleAPI::RoleId).integer().not_null())
                    .col(ColumnDef::new(RoleAPI::APIId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_api_role_id")
                            .from(RoleAPI::Table, RoleAPI::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_api_api_id")
                            .from(RoleAPI::Table, RoleAPI::APIId)
                            .to(API::Table, API::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )

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
pub enum RoleAPI{
    Table,
    Id,
    RoleId,
    APIId,
}
