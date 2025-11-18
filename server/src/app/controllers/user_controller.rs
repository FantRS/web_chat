use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::{
    app::{models::users::UserRequest, request_error::ReqResult, services::user_service},
    core::app_data::AppData,
};

pub async fn get_user(
    user_id: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> ReqResult<impl Responder> {
    let user_id = user_id.into_inner();
    let app_data = app_data.into_inner();

    let user = user_service::get_user(user_id, &app_data.pool).await?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn create_user(
    user: web::Json<UserRequest>,
    app_data: web::Data<AppData>,
) -> ReqResult<impl Responder> {
    let user = user.into_inner().try_into()?;
    let app_data = app_data.into_inner();

    let user_id = user_service::create_user(user, &app_data.pool).await?;

    Ok(HttpResponse::Created().body(user_id.to_string()))
}

pub async fn delete_user(
    user_id: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> ReqResult<impl Responder> {
    let user_id = user_id.into_inner();
    let app_data = app_data.into_inner();



    Ok(HttpResponse::Ok())
}
