use crate::api::account::dtos::create_account_dto::CreateAcountDto;
use crate::handler::errors::CustomError;

use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
// use serde::*;
use uuid::Uuid;
use validator::Validate;

use entity::user;
// use migration::DbErr;
// use sea_orm::{query::*, ActiveModelTrait, ColumnTrait, ConnectionTrait, ModelTrait, Set};

#[post("/signup")]
pub async fn user_signup(
    conn: web::Data<DatabaseConnection>,
    data: web::Json<CreateAcountDto>,
) -> Result<HttpResponse, CustomError> {
    // Validate the input data
    if let Err(e) = data.validate() {
        return Err(CustomError::ValidationError { e });
    };

    let name = data.name.clone();
    let email = data.email.clone();
    let password = data.password.clone();
    let is_admin = data.is_admin;
    let uuid = Uuid::new_v4();
    let time_date = Utc::now().naive_utc();

    println!("uuid :         {}", uuid);

    let new_user = user::ActiveModel {
        name: ActiveValue::Set(name.to_string()),
        email: ActiveValue::Set(email.to_string()),
        password: ActiveValue::Set(password.to_string()),
        uuid: ActiveValue::Set(uuid),
        is_admin: ActiveValue::Set(is_admin),
        created_at: ActiveValue::Set(time_date),
        ..Default::default()
    };

    println!("new user input :         {:#?}", new_user);

    // ! convert type of "conn" i.e app-data into  required type i.e. : &DatabaseConnection or &DbConn
    // let db = &conn as &DatabaseConnection; //method-1
    let db = conn.as_ref(); //method-2

    let _res = user::Entity::insert(new_user).exec(db);
    // .map_err(|e| match e {
    //     DbErr::Query(..) => CustomError::Conflict,
    //     _ => CustomError::ServerError,
    // });

    // let x  = res.try_into().unwrap();

    // let res = user::Entity::insert(user::ActiveModel {
    //     name: ActiveValue::Set(name.to_string()),
    //     email: ActiveValue::Set(email.to_string()),
    //     password: ActiveValue::Set(password.to_string()),
    //     uuid: ActiveValue::Set(uuid),
    //     is_admin: ActiveValue::Set(is_admin),
    //     ..Default::default()
    // })
    // .exec(db)
    // .await
    // .map_err(|e| match e {
    //     DbErr::Query(..) => CustomError::Conflict,
    //     _ => CustomError::ServerError,
    // })?;

    Ok(HttpResponse::Created().body("body"))
}
