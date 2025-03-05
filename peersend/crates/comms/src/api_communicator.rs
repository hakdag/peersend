use core::{api::ApiAccess, create_user::CreateUserRequest, login::LoginRequest, token::TokenStorageAccessable};
use std::{io::{Error, ErrorKind}, str::FromStr};

use ureq::{http::Uri, Error as UreqError};

pub struct APICommunicator<TTokenAccess>
    where TTokenAccess: TokenStorageAccessable {
    token_access: TTokenAccess,
    server_address: String
}

impl<TTokenAccess> APICommunicator<TTokenAccess> where TTokenAccess: TokenStorageAccessable {
    pub fn new(token_access: TTokenAccess, server_address: String) -> Self {
        Self { token_access, server_address }
    }
}

impl<TTokenAccess> ApiAccess for APICommunicator<TTokenAccess> where TTokenAccess: TokenStorageAccessable {
    fn get_target_ipaddress(&self, device_name: &String) -> Result<String, std::io::Error> {
        // read token and send it via web request
        let token = match self.token_access.read() {
            Ok(t) => t,
            Err(e) => return Err(e),
        };
        
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
        let token = match self.token_access.read() {
            Ok(t) => t,
            Err(e) => return Err(e),
        };
        
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
        let uri = match Uri::from_str(&format!("{}/user", self.server_address)) {
            Ok(u) => u,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid server address provided.".to_string())),
        };
        match ureq::post(uri)
            .header("content-type", "application/json")
            .send(serde_json::to_string(&request).unwrap()) {
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
    
    fn login(&self, login_request: LoginRequest) -> Result<String, Error> {
        let uri = match Uri::from_str(&format!("{}/authenticate", self.server_address)) {
            Ok(u) => u,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid server address provided.".to_string())),
        };
        let mut response = match ureq::post(uri)
            .header("content-type", "application/json")
            .send(serde_json::to_string(&login_request).unwrap()) {
                Ok(body) => body,
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
                Ok(token) => Ok(token),
                Err(_) => Err(Error::new(ErrorKind::Other, "Could not read the response from server.")),
            }
        }
}
