use std::io::Error;

use core::{api::ApiAccess, command::Command, requests::login::LoginRequest, token::TokenStorageAccessable};
use crate::{get_arg, get_mac};

pub struct LoginService<TApiAccess, TFile> where TApiAccess: ApiAccess, TFile: TokenStorageAccessable {
    api_access: TApiAccess,
    token_storage_access: TFile
}

impl<TApiAccess, TFile> LoginService<TApiAccess, TFile> where TApiAccess: ApiAccess, TFile: TokenStorageAccessable {
    pub fn new(api_access: TApiAccess, token_storage_access: TFile) -> Self {
        Self { api_access, token_storage_access }
    }

    pub fn run(&self, command: &Command) -> Result<String, Error> {
        let arguments = match &command.arguments {
            Some(args) => args,
            None => &Vec::new(),
        };
        let username = get_arg(arguments, 0);
        let password = get_arg(arguments, 1);
        let mac = get_mac()?;
        let login_request = LoginRequest::new(username, password, mac);
        let token = self.api_access.login(login_request)?;

        match self.token_storage_access.save(token) {
            Ok(_) => Result::Ok("Login successful!".to_string()),
            Err(e) => Err(e),
        }
    }
}