use std::io::{Error, ErrorKind};

use actix_web::HttpRequest;
use services::jwt::token_handler::TokenHandler;

pub mod ip_address;
pub mod user;
pub mod authenticate;
pub mod device;

pub(crate) fn get_token(req: HttpRequest) -> Option<String> {
    match req.headers().get("PS-Token") {
        Some(t) => Some(String::from(t.to_str().unwrap())),
        None => None,
    }
}

pub fn validate_token_and_get_user_id(req: HttpRequest) -> Result<String, Error> {
    let token_raw = get_token(req);
    let token: String;
    if token_raw.is_some() {
        token = token_raw.unwrap();
    }
    else {
        return Err(Error::new(ErrorKind::InvalidData, "Token not found. Please login first."));
    }

    // validate token
    let token_handler = TokenHandler::new();
    // get user id from token
    match token_handler.validate(token) {
        Ok(id) => Ok(id),
        Err(e) => Err(e),
    }
}
