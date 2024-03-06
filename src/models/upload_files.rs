use std::{collections::HashMap, path::PathBuf};

#[derive(serde::Serialize)]
pub enum ContentDisposition {
    Inline,
    Attachment,
}

#[derive(serde::Serialize)]
pub enum ACL {
    Private,
    PublicRead,
}

#[derive(Debug)]
pub struct FileObj {
    pub name: String,
    pub path: PathBuf,
}

#[derive(serde::Serialize)]
pub struct UploadFileOpts {
    pub metadata: Option<HashMap<String, String>>,
    #[serde(rename(serialize = "contentDisposition"))]
    pub content_disposition: Option<ContentDisposition>,
    pub acl: Option<ACL>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct UploadFileResponse {
    pub data: Vec<UploadFileResponseData>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct UploadFileResponseData {
    pub fields: serde_json::Value,
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    pub key: String,
    #[serde(rename = "presignedUrl", default = "default_string")]
    pub presigned_url: String,
    pub url: Option<String>,
    pub urls: Option<Vec<String>>,
    pub chunk_size: Option<u64>,
}

fn default_string() -> String {
    String::new()
}

#[derive(Debug, serde::Deserialize)]
pub struct Fields {
    #[serde(rename = "Content-Disposition", default = "default_string")]
    pub content_disposition: String,
    #[serde(rename = "Content-Type", default = "default_string")]
    pub content_type: String,
    #[serde(default = "default_string")]
    pub policy: String,
    #[serde(rename = "X-Amz-Algorithm", default = "default_string")]
    pub x_amz_algorithm: String,
    #[serde(rename = "X-Amz-Credential", default = "default_string")]
    pub x_amz_credential: String,
    #[serde(rename = "X-Amz-Date", default = "default_string")]
    pub x_amz_date: String,
    #[serde(rename = "X-Amz-Signature", default = "default_string")]
    pub x_amz_signature: String,
    #[serde(default = "default_string")]
    pub acl: String,
    #[serde(default = "default_string")]
    pub bucket: String,
    #[serde(default = "default_string")]
    pub key: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct FileUpload {
    pub key: String,
    pub url: String,
    pub name: String,
    pub size: u64,
}
