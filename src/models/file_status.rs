use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum UploadthingFileStatus {
    DeletionPending,
    Failed,
    Uploaded,
    Uploading,
}