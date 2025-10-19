use axum::http::HeaderValue;
use reqwest::{Method, header};
use tower_http::cors::CorsLayer;

use crate::config::{DotEnvyConfig, Stage};

pub fn create_from_stage(stage: Stage, config: &DotEnvyConfig) -> CorsLayer {
    match stage {
        Stage::Local => create_cors_local_layer(config),
        Stage::Production => create_cors_production_layer(config),
        Stage::Development => create_cors_development_layer(config),
    }
}

pub fn create_cors_local_layer(config: &DotEnvyConfig) -> CorsLayer {
    CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
        .allow_origin(
            config
                .frontend
                .development_url
                .parse::<HeaderValue>()
                .unwrap(),
        )
}

pub fn create_cors_production_layer(config: &DotEnvyConfig) -> CorsLayer {
    CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
        .allow_origin(
            config
                .frontend
                .production_url
                .parse::<HeaderValue>()
                .unwrap(),
        )
}

pub fn create_cors_development_layer(config: &DotEnvyConfig) -> CorsLayer {
    CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
        .allow_origin(
            config
                .frontend
                .development_url
                .parse::<HeaderValue>()
                .unwrap(),
        )
}
