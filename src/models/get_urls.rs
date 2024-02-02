use serde::Deserialize;

// Represents a single upload URL and its associated key.
#[derive(Debug, Deserialize)]
pub struct UploadthingUrl {
    /// The URL to which the thing should be uploaded.
    pub url: String,
    /// The key associated with the uploaded thing, used for referencing it.
    pub key: String,
}

// A response containing a list of upload URLs and their keys.
#[derive(Debug, Deserialize)]
pub struct UploadthingUrlsResponse {
    /// A vector of `UploadthingUrl` objects, each containing a URL and key.
    pub data: Vec<UploadthingUrl>,
}
