use core::requests::login::LoginRequest;
use services::jwt::token_handler::TokenHandler;
use crate::accesses::file::FileAccess;

use actix_web::{web, HttpRequest, HttpResponse};
use validify::Validate;


pub async fn authenticate(_: HttpRequest, body: web::Json<LoginRequest>) -> HttpResponse {
    let login_request = body.0;
    let res = login_request.validate();
    if res.is_err() {
        let err = res.unwrap_err();
        let errs = err.field_errors();
        println!("Login Validation error: {}", errs.first().unwrap());
        return HttpResponse::BadRequest().body(format!("Invalid {} entered.", errs[0].field_name().unwrap()).to_string());
    }

    let fs = FileAccess::new();
    let _ = match fs.get_user_password(&login_request) {
        Ok(password) => {
            if password != login_request.password {
                return HttpResponse::BadRequest().body("Invalid password entered.".to_string());
            }
        },
        Err(e) => {
            println!("Error: {}", e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        },
    };
    
    let email = login_request.email.to_owned();
    let user = match fs.read_user(email) {
        Ok(u) => u,
        Err(e) => {
            println!("Error: {}", e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        },
    };

    // find the matching device from the user devices by mac
    let index = user.devices.iter().position(|d| d.mac.is_some() && d.mac.to_owned().unwrap() == login_request.mac);
    if index.is_none() {
        let token = generate_token(user.email, login_request.mac, None);

        return HttpResponse::Ok().body(token);
    }

    let device_name = match user.devices.get(index.unwrap()) {
        Some(d) => Some(d.devicename.to_owned()),
        None => None,
    };

    // add device to the token
    let token = generate_token(user.email, login_request.mac, device_name);

    HttpResponse::Ok().body(token)

}

fn generate_token(email: String, mac: String, device_name: Option<String>) -> String {
    let token_handler = TokenHandler::new();
    let token = token_handler.generate(&email, Some(mac), device_name).unwrap();
    token
}