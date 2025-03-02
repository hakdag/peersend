use std::io::Error;
use core::{command::Command, storage::StorageAccess, token::TokenStorageAccessable, user::User};
use crate::{get_arg, jwt::token_handler::TokenHandler};

pub struct LoginService<TRedis, TFile> where TRedis: StorageAccess, TFile: TokenStorageAccessable {
    storage_access: TRedis,
    token_storage_access: TFile
}

impl<TRedis, TFile> LoginService<TRedis, TFile> where TRedis: StorageAccess, TFile: TokenStorageAccessable {
    pub fn new(storage_access: TRedis, token_storage_access: TFile) -> Self {
        Self { storage_access, token_storage_access }
    }

    pub fn run(&self, command: &Command) -> Result<String, Error> {
        let arguments = match &command.arguments {
            Some(args) => args,
            None => &Vec::new(),
        };
        let username = get_arg(arguments, 0);
        let password = get_arg(arguments, 1);
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
    }
}