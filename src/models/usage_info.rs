use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UploadthingUsageInfo {
    pub total_bytes: i64,
    pub total_readable: String,
    pub app_total_bytes: f32,
    pub app_total_readable: String,
    pub files_uploaded: i32,
    pub limit_bytes: f32,
    pub limit_readable: String,
}