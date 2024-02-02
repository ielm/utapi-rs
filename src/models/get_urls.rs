use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UploadthingUrl {
    pub url: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadthingUrlsResponse {
    pub data: Vec<UploadthingUrl>,
}
