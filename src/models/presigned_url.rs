use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PresignedUrlOpts {
    pub file_key: String,
    pub expires_in: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PresignedUrlResponse {
    pub url: String,
}
