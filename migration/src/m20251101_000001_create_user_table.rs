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
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Email).not_null().unique_key())
                    .col(string(User::Password).not_null())
                    .col(string(User::Name))
                    .col(ColumnDef::new(User::Role).integer().not_null().default(1))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role")
                            .from(User::Table, User::Role)
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
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User{
    Table,
    Id,
    Email,
    Password,
    Name,
    Role,
}
