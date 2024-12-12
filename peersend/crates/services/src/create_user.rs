use std::io::Error;
use core::{command::Command, storage::StorageAccess, user::User};
use validify::Validate;

use comms::redis_communication::RedisCommunication;
use crate::get_arg;

pub struct CreateUserService {}

impl CreateUserService {
    pub fn run(command: &Command) -> Result<String, Error> {
        let rc = match RedisCommunication::new() {
            Ok(rc) => rc,
            Err(e) => return Err(e),
        };
        let arguments = match &command.arguments {
            Some(args) => args,
            None => &Vec::new(),
        };
        let key = get_arg(arguments, 0);
        let username = get_arg(arguments, 0);
        let user = User::new(username.clone(), get_arg(arguments, 1), get_arg(arguments, 2));

        let res = user.validate();
        if res.is_err() {
            let err = res.unwrap_err();
            let errs = err.field_errors();
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Invalid {} entered.", errs[0].field_name().unwrap()).to_string()));
        }

        match rc.set(key, user) {
            Ok(_) => Result::Ok(format!("User with username '{}' is created.", username)),
            Err(e) => Result::Err(e),
        }
    }
}