use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Serialize, Deserialize, Validify, Debug)]
pub struct RegisterDeviceRequest {
    #[validate(length(min = 3))]
    pub devicename: String,
}

impl RegisterDeviceRequest {
    pub fn new(devicename: String) -> Self {
        Self { devicename }
    }
}
