use serde::{Deserialize, Serialize};
use validify::Validify;

#[derive(Serialize, Deserialize, Validify, Debug)]
pub struct RegisterDeviceRequest {
    #[validate(length(min = 3))]
    pub devicename: String,
    pub mac: String
}

impl RegisterDeviceRequest {
    pub fn new(devicename: String, mac: String) -> Self {
        Self { devicename, mac }
    }
}
