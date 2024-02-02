use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteFileResponse {
    pub success: bool,
}
