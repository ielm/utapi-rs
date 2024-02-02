use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadthingConfig {
    pub host: String,
    pub user_agent: Option<String>,
    pub api_key: Option<String>,
    pub version: Option<String>,
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl UploadthingConfig {
    pub fn new() -> UploadthingConfig {
        UploadthingConfig::default()
    }

    pub fn builder() -> UploadthingConfigBuilder {
        UploadthingConfigBuilder::new()
    }
}

impl Default for UploadthingConfig {
    fn default() -> UploadthingConfig {
        UploadthingConfig {
            host: "https://uploadthing.com".to_string(),
            user_agent: Some(format!("utapi-rs/{}/rust", VERSION).to_owned()),
            api_key: None,
            version: Some(VERSION.to_string()),
        }
    }
}

pub struct UploadthingConfigBuilder {
    config: UploadthingConfig,
}

impl UploadthingConfigBuilder {
    pub fn new() -> Self {
        UploadthingConfigBuilder {
            config: UploadthingConfig::default(),
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.config.host = host.to_string();
        self
    }

    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.config.user_agent = Some(user_agent.to_string());
        self
    }

    pub fn api_key(mut self, api_key: &str) -> Self {
        self.config.api_key = Some(api_key.to_string());
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.config.version = Some(version.to_string());
        self
    }

    pub fn build(self) -> UploadthingConfig {
        self.config
    }
}

impl Default for UploadthingConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
