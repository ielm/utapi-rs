use serde::Serialize;

/// A payload structure representing a list of keys associated with files.
#[derive(Serialize)]
pub struct FileKeysPayload {
    /// A vector of strings where each string is a unique key for a file.
    pub file_keys: Vec<String>,
}

/// Options for listing files, allowing for pagination.
#[derive(Serialize)]
pub struct ListFilesOpts {
    /// The maximum number of files to return in the response.
    /// If not specified, a default limit is applied.
    pub limit: Option<i32>,

    /// The number of files to skip before starting to collect the result set.
    /// If not specified, a default offset is applied.
    pub offset: Option<i32>,
}

impl Default for ListFilesOpts {
    /// Provides default values for `ListFilesOpts`.
    fn default() -> Self {
        ListFilesOpts {
            limit: Some(10), // Default limit set to 10 files.
            offset: Some(0), // Default offset set to 0 to start from the beginning.
        }
    }
}

/// A structure representing a request to rename a single file.
#[derive(Serialize)]
pub struct SingleFileRename {
    /// The unique key of the file to be renamed.
    pub file_key: String,

    /// The new name to be assigned to the file.
    pub new_name: String,
}

/// Options for renaming multiple files in a single operation.
#[derive(Serialize)]
pub struct RenameFilesOpts {
    /// A vector of `SingleFileRename` structures, each representing a single file rename operation.
    pub updates: Vec<SingleFileRename>,
}
