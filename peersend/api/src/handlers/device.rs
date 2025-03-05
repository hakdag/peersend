use core::requests::device::RegisterDeviceRequest;
use services::jwt::token_handler::TokenHandler;
use crate::accesses::file::FileAccess;

use actix_web::{web, HttpRequest, HttpResponse};
use validify::Validate;

use super::validate_token_and_get_user_id;


pub async fn register_device(http_request: HttpRequest, body: web::Json<RegisterDeviceRequest>) -> HttpResponse {
    let request = body.0;
    let res = request.validate();
    if res.is_err() {
        let err = res.unwrap_err();
        let errs = err.field_errors();
        println!("Register device validation error: {}", errs.first().unwrap());
        return HttpResponse::NotModified().body(format!("Invalid {} entered.", errs[0].field_name().unwrap()).to_string());
    }

    // get token from header
    // decrypt token and get user email
    let email = match validate_token_and_get_user_id(http_request) {
        Ok(e) => e,
        Err(e) => {
            println!("Error: {}", e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        },
    };

    // get user from file storage
    let fs = FileAccess::new();
    let user = match fs.read_user(email) {
        Ok(u) => u,
        Err(e) => {
            println!("Error: {}", e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        },
    };

    // check if user has a device wiht same name
    if user.has_device(&request.devicename) {
        return HttpResponse::Found().body("User already has a device with same name.".to_string());
    }

    // if not, add device to devices file
    match fs.add_device_to_user(&user, &request.devicename) {
        Ok(_) => {
            let token_handler = TokenHandler::new();
            let token = token_handler.generate(&user.email).unwrap();
            return HttpResponse::Ok().body(token);
        },
        Err(e) => {
            println!("Error: {}", e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        },
    }
}