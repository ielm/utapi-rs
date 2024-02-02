use serde::Deserialize;

/// Represents the response received from an attempt to delete a file.
///
/// This struct is used to deserialize the JSON response returned by the server
/// when a file deletion operation is performed. The `success` field indicates
/// whether the deletion was successful.
#[derive(Debug, Deserialize)]
pub struct DeleteFileResponse {
    /// A boolean indicating whether the file was successfully deleted.
    pub success: bool,
}
