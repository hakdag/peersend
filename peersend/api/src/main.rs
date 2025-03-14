use actix_web::{guard, web::{self}, App, HttpServer};

use handlers::{authenticate::authenticate, device::register_device, ip_address::{get_ipaddress, set_ipaddress}, user::{create_user, health_check}};
mod models;
mod handlers;
mod accesses;
mod errors;

/*
Have a key-value pair list of target and source ips
target device will first tell its ip address, user id (or name), and device name
source device then will ask for target devices ip by providing device name and user id
*/


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .route("/healthcheck",
                web::get()
                .to(health_check)
            )
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
            .route("/user",
                web::post()
                    .guard(guard::Post())
                    .to(create_user)
            )
            .route("/authenticate",
                web::post()
                    .guard(guard::Post())
                    .to(authenticate)
            )
            .route("/device",
                web::post()
                    .guard(guard::Post())
                    .to(register_device)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}