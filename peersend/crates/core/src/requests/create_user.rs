use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Serialize, Deserialize, Validify, Debug)]
pub struct CreateUserRequest {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password: String,
}

impl CreateUserRequest {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self { username, email, password }
    }
}