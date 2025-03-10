use core::{storage::StorageAccess, token::TokenStorageAccessable, user::{User, UsersAccessable}};
use std::io::Error;

use crate::jwt::token_handler::TokenHandler;

pub struct UserService<TRedis, TTokenAccess>
    where TRedis: StorageAccess, TTokenAccess: TokenStorageAccessable {
        storage_access: TRedis,
        token_storage_access: TTokenAccess
}

impl<TRedis, TTokenAccess> UserService<TRedis, TTokenAccess>
    where TRedis: StorageAccess, TTokenAccess: TokenStorageAccessable {

    pub fn new(storage_access: TRedis,token_storage_access: TTokenAccess) -> Self {
        Self { storage_access, token_storage_access }
    }
}

impl<TRedis, TTokenAccess> UsersAccessable for UserService<TRedis, TTokenAccess>
    where TRedis: StorageAccess, TTokenAccess: TokenStorageAccessable {
        
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

        let user: User = match self.storage_access.get(tui.email) {
            Ok(u) => u,
            Err(e) => return Result::Err(e),
        };

        return Ok(user);
    }
}
