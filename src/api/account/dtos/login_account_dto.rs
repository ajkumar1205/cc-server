// use crate::api::helper::{sanitize_email, trim_string};
use serde::Deserialize;
use validator::Validate;

// !  Santization required
#[derive(Debug, Deserialize, Validate)]
pub struct LoginAcountDto {
    #[validate(email, length(max = 30))]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}
