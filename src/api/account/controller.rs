use crate::api::account::dtos::{login_account_dto::LoginAcountDto, new_user_dto::NewUserDto};
use crate::api::account::service::{insert_user, login_user};
use crate::handler::errors::CustomError;

use actix_web::{web, HttpResponse};
// use futures::TryFutureExt;
use sea_orm::DatabaseConnection;
use uuid::Uuid;
// use sea_orm::{ActiveValue, ColumnTrait, , DbErr, EntityTrait, QueryFilter};
use validator::Validate;

// use entity::user;

// ===== Route-> "/api/signup" ==============================================================
pub async fn user_signup(
    conn: web::Data<DatabaseConnection>,
    data: web::Json<NewUserDto>,
) -> Result<HttpResponse, CustomError> {
    // ! convert type of "conn" i.e app-data into  required type i.e. : &DatabaseConnection or &DbConn
    // let db = &conn as &DatabaseConnection; //method-1
    let db = conn.as_ref(); //method-2

    // Validate the input data
    if let Err(e) = data.validate() {
        return Err(CustomError::ValidationError { e });
    };

    let name = data.name.clone().to_string();
    let email = data.email.clone().to_string();
    let password = data.password.clone().to_string();
    let is_admin = data.is_admin.unwrap_or_default();
    let uuid = Uuid::new_v4();

    let id = insert_user(&db, &name, &email, &password, is_admin, uuid).await?;

    Ok(HttpResponse::Created().json(id))
}

// ===== Route -> "/api/login" post ==============================================================
pub async fn user_login(
    conn: web::Data<DatabaseConnection>,
    data: web::Json<LoginAcountDto>,
) -> Result<HttpResponse, CustomError> {
    let db = conn.as_ref();

    if let Err(e) = data.validate() {
        return Err(CustomError::ValidationError { e });
    };

    let email = data.email.clone().to_string();
    let password = data.password.clone().to_string();

    let login_status = login_user(db, &email, &password).await?;

    if login_status {
        return Ok(HttpResponse::Ok().json("Login Successful"));
    } else {
        return Err(CustomError::Unauthorized);
    }
}
