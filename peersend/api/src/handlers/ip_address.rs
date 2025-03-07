use std::sync::Mutex;

use actix_web::{web, HttpRequest, HttpResponse};
use comms::storage_accesses::redis_communication::RedisCommunication;
use services::jwt::token_handler::TokenHandler;

use crate::{accesses::session::SessionDB, models::peer_session::PeerSession};

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
/*    
    // validate token
    let token_handler = TokenHandler::new();
    // get user id from token
    let user_id = match token_handler.validate(token) {
        Ok(id) => id,
        Err(e) => return HttpResponse::Unauthorized().finish(),
    };
*/
    // read device name from body
    let device_name = match String::from_utf8(body.to_vec()) {
        Ok(text) => text,
        Err(_) => return HttpResponse::BadRequest().body("Invalid device name provided.")
    };

    // get ip address from in-memory db

    /*
    let session = match data.sessions.iter().find(|s| s.device_name == device_name) {
        Some(s) => s,
        None => return HttpResponse::BadRequest().body("Device or Session not found."),
    };
    */

    let session = PeerSession::new("123".to_string(), "1.1.1.1".to_string(), "as:as:as".to_string());

    HttpResponse::Ok().body(session.ip_address.clone())
}

pub async fn set_ipaddress(req: HttpRequest, data: web::Data<Mutex<SessionDB>>, body: web::Bytes) -> HttpResponse {
    // validate token
    // get user id from token
    let (email, mac) = match validate_token_and_get_user_id(req) {
        Ok(e) => e,
        Err(e) => {
            println!("Error: {}", e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        },
    };

    if mac.is_none() {
        return HttpResponse::BadRequest().body("MAC address was not detected in the token. Please re-login.".to_string());
    }

    // save user id, ip address and device name to in-memory db
    let ip_address = match String::from_utf8(body.to_vec()) {
        Ok(text) => text,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ip address provided.")
    };
    let session = PeerSession::new(email, ip_address, mac.unwrap());
    let mut sessionDB = data.lock().unwrap();
    sessionDB.add(session);

    println!("There are {} session(s) in the SessionDB.", sessionDB.count());
    HttpResponse::Ok().finish()
}

