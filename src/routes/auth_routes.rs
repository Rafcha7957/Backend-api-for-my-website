use actix_web::web;

use super::handler;

pub fn config(config: &mut web::ServiceConfig){
    config
        .service(web::scope("/auth")
        );
}