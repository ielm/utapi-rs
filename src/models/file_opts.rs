use serde::Serialize;

#[derive(Serialize)]
pub struct FileKeysPayload {
    pub file_keys: Vec<String>,
}

#[derive(Serialize)]
pub struct ListFilesOpts {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl Default for ListFilesOpts {
    fn default() -> Self {
        ListFilesOpts {
            limit: Some(10),
            offset: Some(0),
        }
    }
}
