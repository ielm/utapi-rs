use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Represents the status of a file being uploaded.
pub enum UploadthingFileStatus {
    /// The file is marked for deletion but not yet deleted.
    DeletionPending,
    /// The file failed to upload.
    Failed,
    /// The file has been successfully uploaded.
    Uploaded,
    /// The file is currently being uploaded.
    Uploading,
}
