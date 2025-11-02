

pub mod setup_user{
    use crate::entities::{prelude::*, };
    use sea_orm::*;
    use crate::entities::dnc_user::{*};
    use password_auth::generate_hash;

    async fn check_admin_user(db: &DatabaseConnection) -> Result<Option<i32>, DbErr> {
        let admin_email = "Administrator".to_owned();
        let existing_admin_user = DncUser::find()
            .filter(Column::Email.eq(&admin_email))
            .one(db)
            .await;
        match existing_admin_user {
            Ok(Some(user)) => {
                let user_clone = user.clone();
                let user_name = user_clone.name;
                let user_id = user_clone.id;
                println!("User {} found to exist in DB with ID: {}", user_name, user_id);
                Ok(Some(user.id))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
    async fn actual_insert_admin_user(db:&DatabaseConnection, admin_role_id:i32) -> Result<i32, DbErr>{
        let admin_name = "Administrator".to_owned();
        let admin_desc = "Administrator".to_owned();
        let raw_password = "<PASSWORD>";
        let stored_hash = generate_hash(raw_password);
        let admin_user = ActiveModel{
            id: NotSet,
            email: Set(admin_name.clone()),
            name: Set(admin_desc),
            password: Set(stored_hash),
            role : Set(admin_role_id),
        };
        let result = DncUser::insert(admin_user).exec(db).await;
        match result {
            Ok(insert_result)=>{
                println!("User '{}' inserted successfully", admin_name.clone());
                Ok(insert_result.last_insert_id)
            }
            Err(e)=>{
                println!("Error inserting user: {}", e);
                Err(e)
            }
        }
    }

    pub async fn insert_admin_user(db:&DatabaseConnection, admin_role_id:i32) ->Result<i32, DbErr> {
        let admin_name = "Administrator".to_owned();
        let existing_admin_user = check_admin_user(db).await;
        match existing_admin_user {
            Ok(Some(user_id))=>{
                println!("User '{}' already exists with ID: {}", admin_name, user_id);
                Ok(user_id)
            },
            Ok(None)=>{
                let insert_result = actual_insert_admin_user(db, admin_role_id).await?;
                Ok(insert_result)
            },
            Err(e)=>{
                println!("Error checking for existing user: {}", e);
                Err(e)
            },
        }
    }

}
