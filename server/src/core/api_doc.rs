use utoipa::OpenApi;

use crate::app::controllers::user_controller;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "web_chat",
        description = "WebChat API documentation", version = "0.1"
    ),
    paths(
        user_controller::get_user,
        user_controller::login_user,
        user_controller::create_user,
        user_controller::patch_user,
        user_controller::delete_user,
    )
)]
pub struct ApiDoc;
