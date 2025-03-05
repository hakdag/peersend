use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Serialize, Deserialize, Validify, Debug)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password: String,
    pub mac: String
}

impl LoginRequest {
    pub fn new(email: String, password: String, mac: String) -> Self {
        Self { email, password, mac }
    }
}