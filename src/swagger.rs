use anyhow::Result;
use utoipa::openapi::OpenApi;

use crate::config;

pub fn create_swagger_ui(api: OpenApi) -> Result<utoipa_swagger_ui::SwaggerUi> {
    let config = config::load()?;
    Ok(
        utoipa_swagger_ui::SwaggerUi::new(config.swagger.swagger_path)
            .url(config.swagger.swagger_json_path, api),
    )
}
