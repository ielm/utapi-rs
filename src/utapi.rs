use reqwest::Client;

use crate::config::UploadthingConfig;

pub struct UtApi {
    config: UploadthingConfig,
    client: Client,
}
