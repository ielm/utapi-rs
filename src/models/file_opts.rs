use serde::Serialize;

#[derive(Serialize)]
pub struct FileKeysPayload {
    pub file_keys: Vec<String>,
}
