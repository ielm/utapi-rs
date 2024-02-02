use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// `UploadthingUsageInfo` holds statistics about the usage of the UploadThing service.
///
/// It contains information about the total bytes transferred, a human-readable representation
/// of the total bytes, application-specific byte counts, the number of files uploaded,
/// and the limits imposed on the usage.
pub struct UploadthingUsageInfo {
    /// The total number of bytes uploaded.
    pub total_bytes: i64,

    /// A human-readable string representing the total number of bytes uploaded.
    pub total_readable: String,

    /// The total number of bytes uploaded attributed to the application level.
    pub app_total_bytes: f32,

    /// A human-readable string representing the application-specific total bytes uploaded.
    pub app_total_readable: String,

    /// The count of uploaded files.
    pub files_uploaded: i32,

    /// The upper limit of bytes that can be uploaded, as a floating-point number.
    pub limit_bytes: f32,

    /// A human-readable string representing the upper limit of bytes that can be uploaded.
    pub limit_readable: String,
}
