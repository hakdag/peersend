use std::io::Error;
use core::{api::ApiAccess, command::Command, login::LoginRequest, token::TokenStorageAccessable};
use crate::get_arg;

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
        let login_request = LoginRequest::new(username, password);
        let token = match self.api_access.login(login_request) {
            Ok(t) => t,
            Err(e) => return Result::Err(e),
        };

        match self.token_storage_access.save(token) {
            Ok(_) => Result::Ok("Login successful!".to_string()),
            Err(e) => Err(e),
        }

        /*
        let result: Result<User, Error> = self.storage_access.get(username);
        match result {
            Ok(user) => {
                if user.password != password {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Invalid password entered.").to_string()));
                }

                let token_handler = TokenHandler::new();
                let token = token_handler.generate(user.username).unwrap();
                match self.token_storage_access.save(token) {
                    Ok(_) => Result::Ok("Login successful!".to_string()),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Result::Err(e),
        }
        */
    }
}