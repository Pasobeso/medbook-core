use anyhow::Result;
use axum::{Router, routing};
use dotenvy::dotenv;
use std::{sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tower_http::{limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

use crate::{app_state::AppState, config, consumers, cors, outbox};

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
    info!("Initialized tracing");
}

pub fn init_env() {
    dotenv().ok();
    info!("Initialized .env");
}

/// Bootstraps a MedBook microservice with common setup steps.
///
/// - Initializes tracing/logging
/// - Loads .env
/// - Creates shared AppState
/// - Starts RabbitMQ consumers
/// - Spawns the outbox worker
/// - Runs the Axum server
pub async fn bootstrap(
    service_name: &str,
    app: Router<AppState>,
    queue_handlers: &[(&str, consumers::ConsumerFn)],
) -> Result<()> {
    let config = config::load()?;
    info!("Config loaded");

    let port = config.server.port;
    let ip = format!("0.0.0.0:{}", port);
    info!("Starting {} on {}...", service_name, ip);

    // Shared app state
    let app_state = AppState::init(&config).await?;
    let shared_state = Arc::new(app_state.clone());

    // Start all message consumers
    for (queue_name, handler) in queue_handlers {
        consumers::init(
            queue_name.to_string(),
            handler.clone(),
            shared_state.clone(),
        );
    }

    let app = app
        .route("/health-check", routing::get(|| async { "OK" }))
        .with_state(app_state)
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout,
        )))
        .layer(RequestBodyLimitLayer::new(config.server.body_limit))
        .layer(TraceLayer::new_for_http())
        .layer(cors::create_from_stage(config::get_stage(), &config));

    info!("Initialized TimeoutLayer");
    info!("Initialized RequestBodyLimitLayer");
    info!("Initialized TraceLayer");
    info!("Initialized CORS with stage: {}", config::get_stage());

    // Start outbox worker
    outbox::init(shared_state.clone());

    // Start the Axum server
    let listener = TcpListener::bind(&ip).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
