use serde::{Deserialize, Serialize};
use validify::Validify;

use crate::device::Device;

#[derive(Serialize, Deserialize, Validify, Debug)]
pub struct CreateUserRequest {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password: String,

    pub devices: Vec<Device>,
}

impl CreateUserRequest {
    pub fn new(username: String, email: String, password: String, devices: Option<Vec<Device>>) -> Self {
        if devices.is_some() {
            Self { username, email, password, devices: devices.unwrap() }
        }
        else {
            Self { username, email, password, devices: Vec::new() }
        }
    }
}