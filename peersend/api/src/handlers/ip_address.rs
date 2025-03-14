use actix_web::{web, HttpRequest, HttpResponse};
use services::jwt::token_handler::TokenHandler;

use crate::{accesses::redis::RedisAccess, models::peer_session::PeerSession};

use super::{get_token, validate_token_and_get_user_id};

pub async fn get_ipaddress(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let token_raw = get_token(req);
    let token: String;
    if token_raw.is_some() {
        token = token_raw.unwrap();
    }
    else {
        return HttpResponse::BadRequest().body("Token not found. Please login first.");
    }

    // validate token
    let token_handler = TokenHandler::new();
    // get user id from token
    let user_info = match token_handler.validate(token) {
        Ok(ui) => ui,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    // read device name from body
    let device_name = match String::from_utf8(body.to_vec()) {
        Ok(text) => text,
        Err(_) => return HttpResponse::BadRequest().body("Invalid device name provided.")
    };

    // get ip address from in-memory db
    let redis_access = match RedisAccess::new() {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let session: PeerSession = match redis_access.get(&user_info.email) {
        Ok(ps) => match ps {
            Some(s) => s,
            None => return HttpResponse::InternalServerError().body("Session could not be found.".to_string()),
        },
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let device = match session.get_device(device_name) {
        Some(d) => d,
        None => return HttpResponse::BadRequest().body("Device not found in the session."),
    };

    HttpResponse::Ok().body(device.ip_address.clone())
}

pub async fn set_ipaddress(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    // validate token
    // get user id from token
    let tui = match validate_token_and_get_user_id(req) {
        Ok(e) => e,
        Err(e) => {
            println!("Error: {}", e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        },
    };

    if tui.mac.is_none() {
        return HttpResponse::BadRequest().body("MAC address was not detected in the token. Please re-login.".to_string());
    }

    // save user id, ip address and device name to in-memory db
    let ip_address = match String::from_utf8(body.to_vec()) {
        Ok(text) => text,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ip address provided.")
    };
    let redis_access = match RedisAccess::new() {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    match redis_access.get::<PeerSession>(&tui.email) {
        Ok(ps) => match ps {
            Some(mut session) => {
                session.add_device(tui.device_name.unwrap(), ip_address, tui.mac.unwrap());

                match redis_access.set(&tui.email, session) {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(_) => HttpResponse::InternalServerError().body("Could not update session.".to_string()),
                }
            },
            None => {
                let session = PeerSession::new(&tui.email, tui.device_name.unwrap(), ip_address, tui.mac.unwrap());

                match redis_access.set(&tui.email, session) {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(_) => HttpResponse::InternalServerError().body("Could not update session.".to_string()),
                }
            }
        },
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }
}

