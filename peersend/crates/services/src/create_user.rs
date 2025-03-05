use std::io::Error;
use core::{api::ApiAccess, command::Command, requests::create_user::CreateUserRequest};

use crate::get_arg;

pub struct CreateUserService<TApiAccess> where TApiAccess: ApiAccess {
    api_access: TApiAccess
}

impl<TApiAccess> CreateUserService<TApiAccess> where TApiAccess: ApiAccess {
    pub fn new(api_access: TApiAccess) -> Self {
        Self { api_access }
    }

    pub fn run(&self, command: &Command) -> Result<String, Error> {
        let arguments = match &command.arguments {
            Some(args) => args,
            None => &Vec::new(),
        };
        let username = get_arg(arguments, 0);
        let user = CreateUserRequest::new(username.clone(), get_arg(arguments, 1), get_arg(arguments, 2));

        match self.api_access.create_user(user) {
            Ok(_) => Result::Ok(format!("User with username '{}' is created.", username)),
            Err(e) => Result::Err(e),
        }
    }
}