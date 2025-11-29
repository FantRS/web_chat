use actix_web::web::{self, ServiceConfig};

use crate::app::controllers::user_controller;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/users/{id}")
            .route(web::get().to(user_controller::get_user))
            .route(web::patch().to(user_controller::patch_user))
            .route(web::delete().to(user_controller::delete_user))
    );
    cfg.service(
        web::resource("/users")
            .route(web::post().to(user_controller::create_user))
    );
    cfg.service(
        web::resource("/users/login/")
            .route(web::get().to(user_controller::login_user))
    );
}
