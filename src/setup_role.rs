
pub mod setup_role {
    use crate::entities::{prelude::*, *};
    use crate::entities::{role, role::Column, role::ActiveModel};
    use sea_orm::*;
    use sea_orm::DbErr;


    /// Returns a search for the admin role.
    async fn check_admin_role(db: &DatabaseConnection) -> Result<Option<role::Model>, DbErr> {
        let admin_name = "Administrator".to_owned();
        let existing_admin_role = Role::find()
            .filter(Column::Name.eq(&admin_name))
            .one(db)
            .await?;
        Ok(existing_admin_role)
    }


    /// Inserts the admin role if it doesn't exist.

    pub async fn insert_unique_admin_role(db: &DatabaseConnection) -> Result<i32, DbErr> {
        let admin_name = "Administrator".to_owned();
        let admin_desc = "Administrator".to_owned();

        // 1. Check if the role already exists
        // We use find_one with a filter on the 'name' column.
        let existing_admin_role = check_admin_role(db).await?;

        // 2. If the role exists, done. Else, insert the role.
        match existing_admin_role {
            Some(role) => {
                println!("Role '{}' already exists with ID: {}", role.name, role.id);
                Ok(role.id)
            }
            None => {
                println!("Role '{}' not found. Inserting new record...", admin_name);
                let admin_role = ActiveModel {
                    id: NotSet,
                    name: Set(admin_name.clone()),
                    desc: Set(admin_desc)
                };
                let result = Role::insert(admin_role).exec(db).await;
                match result {
                    Err(e) => {
                        println!("Error inserting role: {}", e);
                        Err(e)
                    },
                    Ok(insert_result) => {
                        println!("Role '{}' inserted successfully with ID: {}", admin_name, insert_result.last_insert_id);
                        Ok(insert_result.last_insert_id)
                    }
                }
            }
        }
    }

}

