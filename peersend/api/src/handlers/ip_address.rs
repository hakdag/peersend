use std::sync::Mutex;

use actix_web::{web, HttpRequest, HttpResponse};
use services::jwt::token_handler::TokenHandler;

use crate::{accesses::session::SessionDB, models::peer_session::PeerSession};

use super::{get_token, validate_token_and_get_user_id};

pub async fn get_ipaddress(req: HttpRequest, data: web::Data<Mutex<SessionDB>>, body: web::Bytes) -> HttpResponse {
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
    let _ = match token_handler.validate(token) {
        Ok(_) => (),
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    // read device name from body
    let device_name = match String::from_utf8(body.to_vec()) {
        Ok(text) => text,
        Err(_) => return HttpResponse::BadRequest().body("Invalid device name provided.")
    };

    // get ip address from in-memory db
    let session_db = data.lock().unwrap();
    let session = match session_db.get(device_name) {
        Some(s) => s,
        None => return HttpResponse::BadRequest().body("Device or Session not found."),
    };

    HttpResponse::Ok().body(session.ip_address.clone())
}

pub async fn set_ipaddress(req: HttpRequest, data: web::Data<Mutex<SessionDB>>, body: web::Bytes) -> HttpResponse {
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
    let session = PeerSession::new(tui.email, tui.device_name.unwrap(), ip_address, tui.mac.unwrap());
    let mut session_db = data.lock().unwrap();
    session_db.add(session);

    println!("There are {} session(s) in the SessionDB.", session_db.count());
    HttpResponse::Ok().finish()
}

