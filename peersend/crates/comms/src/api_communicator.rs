use core::{api::ApiAccess, requests::{create_user::CreateUserRequest, device::RegisterDeviceRequest, login::LoginRequest}, token::TokenStorageAccessable};
use std::{io::{Error, ErrorKind}, str::FromStr};

use ureq::{http::{Response, Uri}, Body, Error as UreqError};

pub struct APICommunicator<TTokenAccess>
    where TTokenAccess: TokenStorageAccessable {
    token_access: TTokenAccess,
    server_address: String
}

impl<TTokenAccess> APICommunicator<TTokenAccess> where TTokenAccess: TokenStorageAccessable {
    pub fn new(token_access: TTokenAccess, server_address: String) -> Self {
        Self { token_access, server_address }
    }

    pub fn post(&self, path: String, data: String) -> Result<Response<Body>, Error> {
        let uri = match Uri::from_str(&path) {
            Ok(u) => u,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid server address provided.".to_string())),
        };
        match ureq::post(uri)
            .header("content-type", "application/json")
            .send(data) {
                Ok(body) => Ok(body),
                Err(UreqError::StatusCode(code)) => {
                    Err(match code {
                        404 => Error::new(ErrorKind::NotFound, "Resource not found"),
                        500..=599 => Error::new(ErrorKind::Other, "Server error"),
                        _ => Error::new(ErrorKind::Other, format!("HTTP status error: {}", code)),
                    })
                },
                Err(_) => Err(Error::new(ErrorKind::NetworkUnreachable, "Server not reachable"))
        }
    }
}

impl<TTokenAccess> ApiAccess for APICommunicator<TTokenAccess> where TTokenAccess: TokenStorageAccessable {
    fn get_target_ipaddress(&self, device_name: &String) -> Result<String, std::io::Error> {
        // read token and send it via web request
        let token = self.token_access.read()?;
        let uri = match Uri::from_str(&format!("{}/ipaddress/get", self.server_address)) {
            Ok(u) => u,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid server address provided.".to_string())),
        };
        let mut response = match ureq::post(uri)
            .header("PS-Token", token)
            .send(device_name) {
                Ok(r) => r,
                Err(UreqError::StatusCode(code)) => {
                    return Err(match code {
                        404 => Error::new(ErrorKind::NotFound, "Resource not found"),
                        500..=599 => Error::new(ErrorKind::Other, "Server error"),
                        _ => Error::new(ErrorKind::Other, format!("HTTP status error: {}", code)),
                    });
                },
                Err(_) => return Err(Error::new(ErrorKind::NetworkUnreachable, "Server not reachable"))
            };
        match response.body_mut().read_to_string() {
            Ok(b) => Ok(b),
            Err(_) => Err(Error::new(ErrorKind::Other, "Could not read the response from server.")),
        }
    }
    
    fn set_target_ipaddress(&self, ip_address: &String) -> Result<(), std::io::Error> {
        // read token and send it via web request
        let token = self.token_access.read()?;
        let uri = match Uri::from_str(&format!("{}/ipaddress/set", self.server_address)) {
            Ok(u) => u,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid server address provided.".to_string())),
        };

        match ureq::post(uri)
            .header("PS-Token", token)
            .send(ip_address) {
                Ok(_) => Ok(()),
                Err(UreqError::StatusCode(code)) => {
                    return Err(match code {
                        404 => Error::new(ErrorKind::NotFound, "Resource not found"),
                        500..=599 => Error::new(ErrorKind::Other, "Server error"),
                        _ => Error::new(ErrorKind::Other, format!("HTTP status error: {}", code)),
                    });
                },
                Err(_) => return Err(Error::new(ErrorKind::NetworkUnreachable, "Server not reachable"))
            }
    }

    fn create_user(&self, request: CreateUserRequest) -> Result<(), Error> {
        let data = serde_json::to_string(&request).unwrap();
        match self.post(format!("{}/user", self.server_address), data) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    
    fn login(&self, request: LoginRequest) -> Result<String, Error> {
        let data = serde_json::to_string(&request).unwrap();
        let mut response = self.post(format!("{}/authenticate", self.server_address), data)?;
        match response.body_mut().read_to_string() {
            Ok(token) => Ok(token),
            Err(_) => Err(Error::new(ErrorKind::Other, "Could not read the response from server.")),
        }
    }
        
    fn register_device(&self, request: RegisterDeviceRequest) -> Result<String, Error> {
        // read token and send it via web request
        let token = self.token_access.read()?;
        let data = serde_json::to_string(&request).unwrap();
        let uri = match Uri::from_str(&format!("{}/device", self.server_address)) {
            Ok(u) => u,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid server address provided.".to_string())),
        };
        let mut response = match ureq::post(uri)
            .header("PS-Token", token)
            .header("content-type", "application/json")
            .send(data) {
                Ok(r) => r,
                Err(_) => return Err(Error::new(ErrorKind::NetworkUnreachable, "Server not reachable"))
        };
        match response.body_mut().read_to_string() {
            Ok(token) => Ok(token),
            Err(_) => Err(Error::new(ErrorKind::Other, "Could not read the response from server.")),
        }
    }
}
