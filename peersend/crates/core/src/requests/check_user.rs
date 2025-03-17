use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Serialize, Deserialize, Validify, Debug)]
pub struct CheckUserRequest {
    #[validate(length(min = 3))]
    pub target_device: String
}

impl CheckUserRequest {
    pub fn new(target_device: String) -> Self {
        Self { target_device }
    }
}
