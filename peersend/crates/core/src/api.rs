use std::io::Error;

pub trait ApiAccess {
    fn get_target_ipaddress(&self, device_name: &String) -> Result<String, Error>;
    fn set_target_ipaddress(&self, ip_address: &String) -> Result<(), Error>;
}