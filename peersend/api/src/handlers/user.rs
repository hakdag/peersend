use core::{storage::StorageAccess, user::User};

use actix_web::{web, HttpRequest, HttpResponse};
use comms::storage_accesses::redis_communication::RedisCommunication;
use validify::Validate;

pub async fn create_user(req: HttpRequest, body: web::Json<User>) -> HttpResponse {
    let user = body.0;
    println!("Creating user: {}", user.email);
    let res = user.validate();
    if res.is_err() {
        let err = res.unwrap_err();
        let errs = err.field_errors();
        println!("User Validation error: {}", errs.first().unwrap());
        return HttpResponse::BadRequest().body(format!("Invalid {} entered.", errs[0].field_name().unwrap()).to_string());
    }

    let rc = match RedisCommunication::new() {
        Ok(rc) => rc,
        Err(err) => {
            println!("Redis comm error: {}", err);
            return HttpResponse::InternalServerError().body("Storage server not reachable!")
        },
    };
    let storage_access = get_sa(rc);

    let username = user.username.clone();
    match storage_access.set(user.email.clone(), user) {
        Ok(_) => HttpResponse::Created().body(format!("User with username '{}' is created.", username)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

fn get_sa(rc: RedisCommunication) -> impl StorageAccess {
    rc
}

pub async fn health_check() -> HttpResponse {
    println!("Health check received!");
    HttpResponse::Ok().finish()
}