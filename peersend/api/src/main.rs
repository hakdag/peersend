use core::storage::StorageAccess;

use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use comms::storage_accesses::redis_communication::RedisCommunication;
use models::peer_session::PeerSession;
use serde::{Deserialize, Serialize};
use services::jwt::token_handler::TokenHandler;

mod models;

/*
Have a key-value pair list of target and source ips
target device will first tell its ip address, user id (or name), and device name
source device then will ask for target devices ip by providing device name and user id
*/

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
    let rc = match RedisCommunication::new() {
        Ok(rc) => rc,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    /*
    let session = match data.sessions.iter().find(|s| s.device_name == device_name) {
        Some(s) => s,
        None => return HttpResponse::BadRequest().body("Device or Session not found."),
    };
    */

    let session = PeerSession::new("123".to_string(), device_name, "1.1.1.1".to_string());

    HttpResponse::Ok().body(session.ip_address.clone())
}

pub async fn set_ipaddress(req: HttpRequest) -> HttpResponse {
    let token = get_token(req);
    if token.is_some() {
        println!("Header value: {}", token.unwrap());
    }
    else {
        return HttpResponse::BadRequest().body("Token not found. Please login first.");
    }

    // validate token
    // get user id from token
    // save user id, ip address and device name to in-memory db

    HttpResponse::Ok().finish()
}

fn get_token(req: HttpRequest) -> Option<String> {
    match req.headers().get("PS-Token") {
        Some(t) => Some(String::from(t.to_str().unwrap())),
        None => None,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ipaddress/get",
                web::post()
                    .guard(guard::Post())
                    .guard(guard::fn_guard(|ctx| {
                        ctx.head().headers().contains_key("PS-Token")
                    }))
                    .to(get_ipaddress)
            )
            .route("/ipaddress/set",
                web::post()
                    .guard(guard::Post())
                    .guard(guard::fn_guard(|ctx| {
                        ctx.head().headers().contains_key("PS-Token")
                    }))
                    .to(set_ipaddress)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}