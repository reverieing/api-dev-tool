use crate::config::Config;
use crate::error::{Error, Result};
use reqwest::Client as ReqwestClient;

pub struct HttpClient {
    client: ReqwestClient,
    config: Config,
}

impl HttpClient {
    pub fn new(config: Config) -> Result<Self> {
        let client = ReqwestClient::builder()
            .timeout(config.timeout())
            .build()
            .map_err(|e| Error::HttpError(e.to_string()))?;

        Ok(Self { client, config })
    }

    pub fn default_config() -> Result<Self> {
        Self::new(Config::default())
    }

    pub async fn get(&self, url: &str) -> Result<String> {
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::HttpError(e.to_string()))?;

        response
            .text()
            .await
            .map_err(|e| Error::HttpError(e.to_string()))
    }
}