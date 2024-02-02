use serde::{Deserialize, Serialize};

/// A structure representing the options to generate a presigned URL.
///
/// This structure holds the necessary information to create a presigned URL
/// which can be used to access a file without requiring further authentication.
#[derive(Serialize)]
pub struct PresignedUrlOpts {
    /// The unique key of the file for which the presigned URL will be generated.
    pub file_key: String,
    /// Optional expiration time in seconds for the presigned URL.
    /// If `None`, a default value will be used.
    pub expires_in: Option<i32>,
}

/// A structure representing the response received after successfully generating
/// a presigned URL.
///
/// This structure is used to deserialize the response from an API call that
/// generates a presigned URL for accessing a file.
#[derive(Debug, Deserialize)]
pub struct PresignedUrlResponse {
    /// The presigned URL that can be used to access the file.
    pub url: String,
}
