use reqwest::{header, Client, Response};
use serde::Serialize;
use std::error::Error;

use crate::config::{ApiKey, UploadthingConfig};
use crate::models::{
    DeleteFileResponse, FileKeysPayload, ListFilesOpts, PresignedUrlOpts, PresignedUrlResponse,
    RenameFilesOpts, UploadthingFileResponse, UploadthingUrlsResponse, UploadthingUsageInfo,
};

/// The `UtApi` struct represents the client for interacting with the Uploadthing API.
///
/// It contains the configuration for the service and the HTTP client used to make requests.
pub struct UtApi {
    /// Configuration for the Uploadthing service, including the API key and other settings.
    config: UploadthingConfig,

    /// The HTTP client for making requests to the Uploadthing service.
    client: Client,
}

impl UtApi {
    /// Creates a new instance of `UtApi`.
    ///
    /// This constructor initializes the `UtApi` struct with the provided API key
    /// or, if none is provided, attempts to retrieve the API key from the environment.
    /// It sets up the `UploadthingConfig` and the internal `Client` for HTTP requests.
    ///
    /// # Arguments
    ///
    /// * `api_key` - An `Option<String>` that holds the API key for authentication.
    ///               If `None`, the API key is retrieved from the environment.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create a new API client with a provided API key.
    /// let api_with_key = UtApi::new(Some("your_api_key".to_string()));
    ///
    /// // Create a new API client using the API key from the environment.
    /// let api_with_env_key = UtApi::new(None);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a new `UtApi` struct initialized with the provided or environment API key
    /// and a new `Client`.
    ///
    /// # Panics
    ///
    /// Panics if the API key is not provided and is also not set in the environment.
    pub fn new(api_key: Option<String>) -> UtApi {
        // Initialize the configuration for the Uploadthing service using the provided API key.
        // If no API key is provided, attempt to retrieve the key from the environment variable.
        let api_key = api_key.unwrap_or_else(|| {
            ApiKey::from_env()
                .expect("API key not provided and not found in environment")
                .to_string()
        });

        // Build the configuration with the retrieved or provided API key.
        let config = UploadthingConfig::builder().api_key(&api_key).build();

        // Create a new HTTP client for making requests.
        let client = Client::new();

        // Return a new instance of `UtApi` with the configured settings.
        UtApi { config, client }
    }

    /// Creates a new instance of `UtApi` from a given `UploadthingConfig`.
    ///
    /// # Arguments
    ///
    /// * `config` - An `UploadthingConfig` instance containing the configuration for the service.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = UploadthingConfig::builder().api_key("your_api_key").build();
    /// let api = UtApi::from_config(config);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a new `UtApi` struct initialized with the provided configuration and a new `Client`.
    pub fn from_config(config: UploadthingConfig) -> UtApi {
        let client = Client::new();
        UtApi { config, client }
    }

    /// Sends a `POST` request to the `Uploadthing` service.
    ///
    /// This method constructs a URL using the `pathname` and the host from the configuration,
    /// then sends a `POST` request with the provided `payload` serialized as JSON.
    /// It also sets necessary headers, including the user agent, API key, and version,
    /// as well as a `Cache-Control` header to prevent caching.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type of the `payload` that implements `Serialize`.
    ///
    /// # Parameters
    ///
    /// * `pathname`: The endpoint path to which the request will be sent.
    /// * `payload`: The data to be serialized and sent as the request body.
    ///
    /// # Returns
    ///
    /// A `Result` with the HTTP `Response` if the request was successful,
    /// or an `Error` boxed in a `Box<dyn Error>` if the request failed.
    ///
    /// # Errors
    ///
    /// If the response status is not a success, this function will return an `Error`
    /// containing the HTTP error returned by the server.
    pub async fn request_uploadthing<T: Serialize>(
        &self,
        pathname: &str,
        payload: &T,
    ) -> Result<Response, Box<dyn Error>> {
        // Construct the full URL by appending the pathname to the host from the config.
        let url = format!("{}/{}", self.config.host, pathname);

        // Perform a POST request with the serialized payload.
        let response = self
            .client
            .post(&url)
            .json(payload) // Serialize the payload as JSON and set it as the request body.
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CACHE_CONTROL, "no-store") // Ensure the response is not cached.
            .header(header::USER_AGENT, self.config.user_agent.as_ref().unwrap()) // Set the User-Agent header.
            .header(
                "x-uploadthing-api-key",
                self.config.api_key.as_ref().unwrap(), // Set the custom API key header.
            )
            .header(
                "x-uploadthing-version",
                self.config.version.as_ref().unwrap(), // Set the custom version header.
            )
            .send() // Send the request.
            .await?; // Await the async operation, returning an error if one occurs.

        // Check the HTTP response status code to determine success.
        if response.status().is_success() {
            Ok(response) // If successful, return the response.
        } else {
            // If the response indicates failure, extract and return the error.
            Err(Box::new(response.error_for_status().unwrap_err()))
        }
    }

    /// Sends a `DELETE` request to the `Uploadthing` service to delete a list of files.
    ///
    /// This method accepts a list of file keys and constructs a payload to send to the
    /// `/api/deleteFile` endpoint. It then calls the `request_uploadthing` method to
    /// perform the actual request.
    ///
    /// # Parameters
    ///
    /// * `file_keys`: A `Vec<String>` containing the keys of the files to be deleted.
    ///
    /// # Returns
    ///
    /// A `Result` with a `DeleteFileResponse` if the deletion was successful,
    /// or an `Error` boxed in a `Box<dyn Error>` if the request failed.
    ///
    /// # Errors
    ///
    /// If the response status is not a success, or if the response cannot be deserialized
    /// into a `DeleteFileResponse`, this function will return an `Error`.
    pub async fn delete_files(
        &self,
        file_keys: Vec<String>,
    ) -> Result<DeleteFileResponse, Box<dyn Error>> {
        // Construct the payload with the file keys to be deleted.
        let payload = FileKeysPayload { file_keys };

        // Make a `DELETE` request to the Uploadthing service using the constructed payload.
        let response = self
            .request_uploadthing("/api/deleteFile", &payload)
            .await?;

        // Deserialize the JSON response into the `DeleteFileResponse` struct.
        // This holds the result of the delete operation.
        let delete_response: DeleteFileResponse = response.json().await?;

        // Return the deserialized delete response.
        Ok(delete_response)
    }

    /// Retrieves the URLs for a list of file keys from the `Uploadthing` service.
    ///
    /// # Parameters
    ///
    /// * `file_keys`: A `Vec<String>` containing the keys of the files whose URLs are to be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result` with a `UploadthingUrlsResponse` if the retrieval was successful,
    /// or an `Error` boxed in a `Box<dyn Error>` if the request failed.
    ///
    /// # Errors
    ///
    /// If the response status is not a success, or if the response cannot be deserialized
    /// into a `UploadthingUrlsResponse`, this function will return an `Error`.
    pub async fn get_file_urls(
        &self,
        file_keys: Vec<String>,
    ) -> Result<UploadthingUrlsResponse, Box<dyn Error>> {
        // Construct the payload with the file keys for which URLs are to be retrieved.
        let payload = FileKeysPayload { file_keys };

        // Make a `POST` request to the Uploadthing service using the constructed payload.
        // Note: Assuming that the `getFileUrl` API uses a POST method as it was unspecified;
        // adapt the HTTP method according to the API specification if necessary.
        let response = self
            .request_uploadthing("/api/getFileUrl", &payload)
            .await?;

        // Deserialize the JSON response into the `UploadthingUrlsResponse` struct.
        // This holds the URLs for the requested file keys.
        let urls_response: UploadthingUrlsResponse = response.json().await?;

        // Return the deserialized URLs response.
        Ok(urls_response)
    }

    /// Lists files stored in `Uploadthing` service.
    ///
    /// # Parameters
    ///
    /// * `opts`: An optional `ListFilesOpts` struct with parameters to control pagination.
    ///
    /// # Returns
    ///
    /// A `Result` with a `UploadthingFileResponse` if the retrieval was successful,
    /// or an `Error` boxed in a `Box<dyn Error>` if the request failed.
    ///
    /// # Errors
    ///
    /// If the response status is not a success, or if the response cannot be deserialized
    /// into a `UploadthingFileResponse`, this function will return an `Error`.
    pub async fn list_files(
        &self,
        opts: Option<ListFilesOpts>,
    ) -> Result<UploadthingFileResponse, Box<dyn Error>> {
        // You might serialize `None` to send no specific parameters,
        // or provide a default instance of ListFilesOpts with desired default values.
        let payload = opts.unwrap_or_default();

        // Make a `POST` request to the Uploadthing service using the constructed payload.
        let response = self.request_uploadthing("/api/listFiles", &payload).await?;

        // Deserialize the JSON response into the `UploadthingFileResponse` struct.
        let file_response: UploadthingFileResponse = response.json().await?;

        // Return the deserialized file response.
        Ok(file_response)
    }

    /// Renames files in the `Uploadthing` service according to the given options.
    ///
    /// # Parameters
    ///
    /// * `files`: A `RenameFilesOpts` struct with the file keys and new names for renaming.
    ///
    /// # Returns
    ///
    /// An `Ok` result if the renaming operation was successful,
    /// or an `Error` boxed in a `Box<dyn Error>` if the request failed.
    ///
    /// # Errors
    ///
    /// If the response status is not a success, this function will return an `Error`.
    pub async fn rename_files(&self, files: RenameFilesOpts) -> Result<(), Box<dyn Error>> {
        // Make a `POST` request to the Uploadthing service using the constructed payload.
        // No response content is expected based on the comment in the Go code.
        let _response = self.request_uploadthing("/api/renameFiles", &files).await?;

        // If successful, return an `Ok` result with no value.
        Ok(())
    }

    /// Gets usage information for the current `Uploadthing` account.
    ///
    /// # Returns
    ///
    /// A `Result` with a `UploadthingUsageInfo` if the retrieval was successful,
    /// or an `Error` boxed in a `Box<dyn Error>` if the request failed.
    ///
    /// # Errors
    ///
    /// If the response status is not a success, or if the response cannot be deserialized
    /// into an `UploadthingUsageInfo`, this function will return an `Error`.
    pub async fn get_usage_info(&self) -> Result<UploadthingUsageInfo, Box<dyn Error>> {
        // Make a `GET` request to the Uploadthing service to get the usage info.
        // An empty payload is assumed because of the "bytes.NewBuffer([]byte{})" in Go code.
        let response = self.request_uploadthing("/api/getUsageInfo", &()).await?;

        // Deserialize the JSON response into the `UploadthingUsageInfo` struct.
        let usage_info: UploadthingUsageInfo = response.json().await?;

        // Return the deserialized usage information.
        Ok(usage_info)
    }

    /// Generates a presigned URL for a file.
    ///
    /// The maximum value for `expires_in` is 604800 (7 days).
    /// This function assumes that you must accept overrides on the UploadThing dashboard
    /// for `expires_in` to be accepted.
    ///
    /// # Parameters
    ///
    /// * `opts`: A `PresignedUrlOpts` struct containing options for the presigned URL,
    ///           including the file key and the expiration time in seconds.
    ///
    /// # Returns
    ///
    /// A `Result` with a `String` presigned URL if the operation was successful,
    /// or an `Error` boxed in a `Box<dyn Error>` if the request failed, including
    /// scenarios where `expires_in` is greater than the allowed maximum.
    ///
    /// # Errors
    ///
    /// If `expires_in` is greater than 604800 or if an error occurs during the request,
    /// an `Error` is returned.
    pub async fn get_presigned_url(
        &self,
        opts: PresignedUrlOpts,
    ) -> Result<String, Box<dyn Error>> {
        // Validate expiresIn.
        if opts.expires_in.unwrap_or(0) > 604800 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "expiresIn must be less than 604800",
            )));
        }

        // Make a `POST` request to the Uploadthing service using the constructed payload.
        let response = self
            .request_uploadthing("/api/requestFileAccess", &opts)
            .await?;

        // Deserialize the JSON response into the `PresignedUrlResponse` struct.
        let url_response: PresignedUrlResponse = response.json().await?;

        // Return the `url` from the deserialized response.
        Ok(url_response.url)
    }
}
