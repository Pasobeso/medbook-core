use anyhow::Result;
use reqwest::Client;
use rmq_wrappers::Rmq;

use crate::{
    config::DotEnvyConfig,
    db::{self, DbPool},
};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub http_client: Client,
    pub rmq_client: Rmq,
}

impl AppState {
    pub async fn init(config: &DotEnvyConfig) -> Result<Self> {
        Ok(Self {
            db_pool: db::connect(&config.database.url).await?,
            http_client: Client::new(),
            rmq_client: Rmq::connect(&config.message_queue.url).await?,
        })
    }
}
