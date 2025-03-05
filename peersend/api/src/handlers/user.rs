use core::requests::create_user::CreateUserRequest;

use actix_web::{web, HttpRequest, HttpResponse};
use validify::Validate;

use crate::accesses::file::FileAccess;

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

pub async fn health_check() -> HttpResponse {
    println!("Health check received!");
    HttpResponse::Ok().finish()
}