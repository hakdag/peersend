use core::login::LoginRequest;
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
    match fs.get_user_password(&login_request) {
        Ok(password) => {
            if password != login_request.password {
                return HttpResponse::BadRequest().body("Invalid password entered.".to_string());
            }

            let token_handler = TokenHandler::new();
            let token = token_handler.generate(login_request).unwrap();
            HttpResponse::Ok().body(token)
        },
        Err(e) => {
            println!("Error: {}", e.to_string());
            HttpResponse::InternalServerError().body(e.to_string())
        },
    }
}