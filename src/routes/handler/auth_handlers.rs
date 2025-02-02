use actix_web::{post, web, Responder};
use sea_orm::Set;
use crate::utils::{api_response, app_state};

#[derive(serde::Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String,
}

#[post("register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>
) -> impl Responder{

    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        name: Set(register_json.email.clone()),
        name: Set(register_json.password.clone()),
        ..Default::default()
    };

    api_response::ApiResponse::new(200, "".to_string())

}