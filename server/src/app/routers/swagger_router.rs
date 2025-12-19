use actix_web::web::ServiceConfig;
use utoipa::OpenApi;

use crate::core::api_doc::ApiDoc;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
