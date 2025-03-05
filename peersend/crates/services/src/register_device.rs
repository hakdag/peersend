use std::io::Error
;
use core::{api::ApiAccess, command::Command, device::Device, requests::device::RegisterDeviceRequest, token::TokenStorageAccessable};
use crate::{get_arg, get_mac};

pub struct RegisterDeviceService<TApiAccess, TFile> where TApiAccess: ApiAccess, TFile: TokenStorageAccessable {
    api_access: TApiAccess,
    token_storage_access: TFile
}

impl<TApiAccess, TFile> RegisterDeviceService<TApiAccess, TFile> where TApiAccess: ApiAccess, TFile: TokenStorageAccessable {
    pub fn new(api_access: TApiAccess, token_storage_access: TFile) -> Self {
        Self { api_access, token_storage_access }
    }

    pub fn run(&self, command: &Command) -> Result<String, Error> {
        // create device obj
        let arguments = match &command.arguments {
            Some(args) => args,
            None => &Vec::new(),
        };

        let mac = get_mac()?;
        let device = Device::new(get_arg(arguments, 0), None);
        let request = RegisterDeviceRequest::new(device.devicename, mac);

        let token = self.api_access.register_device(request)?;

        match self.token_storage_access.save(token) {
            Ok(_) => Result::Ok("Device is registered.".to_string()),
            Err(e) => Err(e),
        }
    }
}