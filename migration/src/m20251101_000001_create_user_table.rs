// m20251101_000001_create_user_table.rs
use sea_orm_migration::{prelude::*, schema::*};
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
                    .table(DNCUser::Table)
                    .if_not_exists()
                    .col(pk_auto(DNCUser::Id))
                    .col(string(DNCUser::Email).not_null().unique_key())
                    .col(string(DNCUser::Password).not_null())
                    .col(string(DNCUser::Name))
                    .col(ColumnDef::new(DNCUser::Role).integer().not_null().default(1))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role")
                            .from(DNCUser::Table, DNCUser::Role)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),

            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migrationxxxx scripts

        manager
            .drop_table(Table::drop().table(DNCUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DNCUser{
    Table,
    Id,
    Email,
    Password,
    Name,
    Role,
}
