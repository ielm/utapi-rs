// Module for handling file deletion operations.
pub mod delete_file;
// Exports the `DeleteFileResponse` type for external use.
pub use delete_file::DeleteFileResponse;

// Module for defining options related to file operations.
pub mod file_opts;
// Exports types related to file keys payload, listing, and renaming options.
pub use file_opts::{FileKeysPayload, ListFilesOpts, RenameFilesOpts, SingleFileRename};

// Module for retrieving URLs for uploading.
pub mod get_urls;
// Exports the `UploadthingUrlsResponse` type for external use.
pub use get_urls::UploadthingUrlsResponse;

// Module for listing files.
pub mod list_files;
// Exports types related to the response of listing files.
pub use list_files::{UploadthingFile, UploadthingFileResponse};

// Module for checking the status of a file.
pub mod file_status;
// Exports the `UploadthingFileStatus` type for external use.
pub use file_status::UploadthingFileStatus;

// Module for retrieving usage information.
pub mod usage_info;
// Exports the `UploadthingUsageInfo` type for external use.
pub use usage_info::UploadthingUsageInfo;

// Module for generating presigned URLs for secure file access.
pub mod presigned_url;
// Exports types related to presigned URL options and responses.
pub use presigned_url::{PresignedUrlOpts, PresignedUrlResponse};

pub mod upload_files;
pub use upload_files::{
    ContentDisposition, FileObj, FileUpload, UploadFileOpts, UploadFileResponse,
    UploadFileResponseData, ACL,
};
