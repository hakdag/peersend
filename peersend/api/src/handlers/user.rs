use core::requests::{check_user::CheckUserRequest, create_user::CreateUserRequest};

use actix_web::{web, HttpRequest, HttpResponse};
use validify::Validate;

use crate::accesses::file::FileAccess;

use super::validate_token_and_get_user_id;

pub async fn create_user(_: HttpRequest, body: web::Json<CreateUserRequest>) -> HttpResponse {
    let request = body.0;
    println!("Creating user: {}", request.email);
    let res = request.validate();
    if res.is_err() {
        let err = res.unwrap_err();
        let errs = err.field_errors();
        println!("User Validation error: {}", errs.first().unwrap());
        return HttpResponse::BadRequest().body(format!("Invalid {} entered.", errs[0].field_name().unwrap()).to_string());
    }

    let fs = FileAccess::new();
    let username = request.username.clone();
    match fs.write_user(request) {
        Ok(_) => HttpResponse::Created().body(format!("User with username '{}' is created.", username)),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn check_user(http_request: HttpRequest, body: web::Json<CheckUserRequest>) -> HttpResponse {
    let user_info = match validate_token_and_get_user_id(http_request) {
        Ok(tui) => tui,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let request = body.0;
    let res = request.validate();
    if res.is_err() {
        let err = res.unwrap_err();
        let errs = err.field_errors();
        println!("User Validation error: {}", errs.first().unwrap());
        return HttpResponse::BadRequest().body(format!("Invalid {} entered.", errs[0].field_name().unwrap()).to_string());
    }

    let fs = FileAccess::new();
    let user = match fs.read_user(user_info.email) {
        Ok(u) => u,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };
    if !user.has_device(&user_info.device_name.unwrap()) {
        return HttpResponse::BadRequest().body("User did not register the source device.".to_string());
    }
    if !user.has_device(&request.target_device) {
        return HttpResponse::BadRequest().body("User did not register the target device.".to_string());
    }

    HttpResponse::Ok().finish()
}

pub async fn health_check() -> HttpResponse {
    println!("Health check received!");
    HttpResponse::Ok().finish()
}