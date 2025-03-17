use std::io::Error;

use crate::{requests::{check_user::CheckUserRequest, create_user::CreateUserRequest, device::RegisterDeviceRequest, login::LoginRequest}, user::User};

pub trait ApiAccess {
    fn get_target_ipaddress(&self, device_name: &String) -> Result<String, Error>;
    fn set_target_ipaddress(&self, ip_address: &String) -> Result<(), Error>;
    fn create_user(&self, request: CreateUserRequest) -> Result<(), Error>;
    fn login(&self, request: LoginRequest) -> Result<String, Error>;
    fn register_device(&self, request: RegisterDeviceRequest) -> Result<String, Error>;
    fn get_user(&self, email: String) -> Result<User, Error>;
    fn check_user(&self, request: CheckUserRequest) -> Result<(), Error>;
}