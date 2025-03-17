use core::{api::ApiAccess, requests::check_user::CheckUserRequest, token::TokenStorageAccessable, user::{User, UsersAccessable}};
use std::io::Error;

use crate::jwt::token_handler::TokenHandler;

pub struct UserService<TTokenAccess, TApiAccess>
    where TTokenAccess: TokenStorageAccessable, TApiAccess: ApiAccess {
        token_storage_access: TTokenAccess,
        api_access: TApiAccess
}

impl<TTokenAccess, TApiAccess> UserService<TTokenAccess, TApiAccess>
    where TTokenAccess: TokenStorageAccessable, TApiAccess: ApiAccess {

    pub fn new(token_storage_access: TTokenAccess, api_access: TApiAccess) -> Self {
        Self { token_storage_access, api_access }
    }
}

impl<TTokenAccess, TApiAccess> UsersAccessable for UserService<TTokenAccess, TApiAccess>
    where TTokenAccess: TokenStorageAccessable, TApiAccess: ApiAccess {
        
    fn get_token(&self) -> Result<String, Error> {
        self.token_storage_access.read()
    }

    fn get_user(&self) -> Result<User, Error> {
        // read token
        let token = match self.get_token() {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        // validate token
        // get user Id from token's sub claim
        let token_handler = TokenHandler::new();
        let tui = match token_handler.validate(token) {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        let user: User = match self.api_access.get_user(tui.email) {
            Ok(u) => u,
            Err(e) => return Result::Err(e),
        };

        return Ok(user);
    }
    
    fn check_user(&self, request: CheckUserRequest) -> Result<(), Error> {
        // read token
        let token = match self.get_token() {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        // validate token
        // get user Id from token's sub claim
        let token_handler = TokenHandler::new();
        let _ = match token_handler.validate(token) {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        match self.api_access.check_user(request) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
