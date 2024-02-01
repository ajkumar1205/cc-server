// use crate::api::helper::{sanitize_email, trim_string};
use serde::Deserialize;
use validator::Validate;

// !  Santization required
#[derive(Debug, Deserialize, Validate)]
pub struct NewUserDto {
    #[validate(length(min = 3, max = 20))]
    pub name: String,

    #[validate(email, length(max = 30))]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(must_match = "password", length(min = 8))]
    pub confirm_password: String,

    pub is_admin: Option<bool>,
}
