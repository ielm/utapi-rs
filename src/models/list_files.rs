use serde::Deserialize;

use crate::models::UploadthingFileStatus;

/// A representation of an uploaded file within the system.
///
/// This struct is used to deserialize information about files that have been
/// uploaded, including their key, unique identifier, and current status.
#[derive(Debug, Deserialize)]
pub struct UploadthingFile {
    /// A unique key associated with the file, typically used for retrieval.
    pub key: String,
    /// A unique identifier for the file, often a UUID.
    pub id: String,
    /// The current status of the file, indicating whether it's pending, completed, etc.
    pub status: UploadthingFileStatus,
}

/// A response structure containing a list of `UploadthingFile` objects.
///
/// This is typically used to send a collection of uploaded file information
/// back to a client, encapsulating the data in a single struct for convenience.
#[derive(Debug, Deserialize)]
pub struct UploadthingFileResponse {
    /// A vector of `UploadthingFile` objects representing each file's data.
    pub files: Vec<UploadthingFile>,
}
