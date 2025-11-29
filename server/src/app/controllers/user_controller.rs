use actix_web::{HttpResponse, Responder, web};
use uuid::Uuid;

use crate::{
    app::{
        models::users::{CreateUserRequest, LoginUserRequest, PatchUserRequest},
        request_error::RequestResult,
        services::user_service,
    },
    core::app_data::AppData,
};

// e1615c0d-eca8-445b-9666-63be1ead3ed0

#[tracing::instrument(name = "get_user", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn get_user(
    user_id: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let user_id = user_id.into_inner();
    let app_data = app_data.into_inner();

    let response = user_service::get_user(user_id, &app_data.pool).await;

    match &response {
        Ok(_) => tracing::info!("User successfully retrieved from database!"),
        Err(e) => tracing::error!("Error: {}", e),
    };

    // UserResponse
    Ok(HttpResponse::Ok().json(response?))
}

#[tracing::instrument(name = "login_user", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn login_user(
    user: web::Json<LoginUserRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let user = user.into_inner().try_into()?;
    let app_data = app_data.into_inner();

    let response = user_service::login_user(user, &app_data.jwt_secret, &app_data.pool).await;

    match &response {
        Ok(_) => tracing::info!("The user has been successfully logged in!"),
        Err(e) => tracing::error!("Error: {}", e),
    };

    // JsonWebToken
    Ok(HttpResponse::Ok().body(response?))
}

#[tracing::instrument(name = "create_user", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn create_user(
    user: web::Json<CreateUserRequest>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let user = user.into_inner().try_into()?;
    let app_data = app_data.into_inner();

    let response = user_service::create_user(user, &app_data.pool).await;

    match &response {
        Ok(_) => tracing::info!("The user has been successfully created!"),
        Err(e) => tracing::error!("Error: {}", e),
    };

    // Uuid
    Ok(HttpResponse::Created().body(response?.to_string()))
}

#[tracing::instrument(name = "patch_user", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn patch_user(
    user: web::Json<PatchUserRequest>,
    user_id: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let user = user.into_inner().try_into()?;
    let user_id = user_id.into_inner();
    let app_data = app_data.into_inner();

    let response = user_service::patch_user(user_id, user, &app_data.pool).await;

    match &response {
        Ok(_) => tracing::info!("The user has been successfully patched!"),
        Err(e) => tracing::error!("Error: {}", e),
    };

    // Uuid
    Ok(HttpResponse::Ok().body(response?.to_string()))
}

#[tracing::instrument(name = "delete_user", skip_all, fields(request_id = %Uuid::new_v4()))]
pub async fn delete_user(
    user_id: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> RequestResult<impl Responder> {
    let user_id = user_id.into_inner();
    let app_data = app_data.into_inner();

    let response = user_service::delete_user(user_id, &app_data.pool).await;

    match &response {
        Ok(_) => tracing::info!("The user has been successfully deleted!"),
        Err(e) => tracing::error!("Error: {e}"),
    };

    // Uuid
    Ok(HttpResponse::Ok().body(response?.to_string()))
}
