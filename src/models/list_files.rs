use serde::Deserialize;

use crate::models::UploadthingFileStatus;

#[derive(Debug, Deserialize)]
pub struct UploadthingFile {
    pub key: String,
    pub id: String,
    pub status: UploadthingFileStatus,
}

#[derive(Debug, Deserialize)]
pub struct UploadthingFileResponse {
    pub files: Vec<UploadthingFile>,
}