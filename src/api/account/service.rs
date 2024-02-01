// use bcrypt::{hash, verify, DEFAULT_COST};
use entity::user;
use migration::DbErr;
// use sea_orm::{query::*, ActiveModelTrait, ConnectionTrait, ModelTrait};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::handler::errors::CustomError;

// ====================== for creating new user
pub async fn insert_user(
    db: &DatabaseConnection,
    name: &str,
    email: &str,
    password: &str,
    is_admin: bool,
    uuid: Uuid,
) -> Result<usize, CustomError> {
    // let password = password.to_string();
    // let hashed_pass = hash(password, DEFAULT_COST).map_err(|e| return e);

    let new_user = user::ActiveModel {
        name: Set(name.to_string()),
        email: Set(email.to_string()),
        // password: Set(hashed_pass.to_string()),
        password: Set(password.to_string()),
        uuid: Set(uuid),
        is_admin: Set(is_admin),
        ..Default::default()
    };

    let res = user::Entity::insert(new_user)
        .exec(db)
        .await
        .map_err(|e| match e {
            DbErr::Query(..) => CustomError::Conflict,
            _ => CustomError::ServerError,
        })?;

    Ok(res.last_insert_id as usize)
}

// ===================== checks for login
pub async fn login_user(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
) -> Result<bool, CustomError> {
    let email = email.to_string();
    let password = password.to_string();

    let result = user::Entity::find()
        .filter(user::Column::Email.contains(email))
        .one(db)
        .await
        .map_err(|_| CustomError::ServerError)?;

    if result.is_none() {
        return Err(CustomError::NotFound);
    };

    let result_pass = result.clone().unwrap().password;

    if password == result_pass {
        Ok(true)
    } else {
        Err(CustomError::Unauthorized)
    }
}
