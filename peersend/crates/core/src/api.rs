use std::io::Error;

use crate::requests::{create_user::CreateUserRequest, device::RegisterDeviceRequest, login::LoginRequest};

pub trait ApiAccess {
    fn get_target_ipaddress(&self, device_name: &String) -> Result<String, Error>;
    fn set_target_ipaddress(&self, ip_address: &String) -> Result<(), Error>;
    fn create_user(&self, request: CreateUserRequest) -> Result<(), Error>;
    fn login(&self, request: LoginRequest) -> Result<String, Error>;
    fn register_device(&self, request: RegisterDeviceRequest) -> Result<String, Error>;
}