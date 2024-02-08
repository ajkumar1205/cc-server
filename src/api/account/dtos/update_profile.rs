// use crate::api::helper::{sanitize_email, trim_string};
use serde::Deserialize;
use validator::Validate;

// !  Santization required
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileDto {
    #[validate(length(min = 3, max = 20))]
    pub name: Option<String>,
    #[Validate(url)]
    pub profile_pic: Option<Vec<u8>>,
}
