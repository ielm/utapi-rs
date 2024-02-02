use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteFileResponse {
    success: bool,
}
