use reqwest::{header, Client, Response};
use serde::Serialize;
use std::error::Error;

use crate::config::UploadthingConfig;
use crate::models::{
    DeleteFileResponse, FileKeysPayload, ListFilesOpts, RenameFilesOpts, UploadthingFileResponse,
    UploadthingUrlsResponse,
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
    /// # Arguments
    ///
    /// * `api_key` - A string slice that holds the API key for authentication.
    ///
    /// # Examples
    ///
    /// ```
    /// let api = UtApi::new("your_api_key");
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a new `UtApi` struct initialized with the provided API key and a new `Client`.
    pub fn new(api_key: &str) -> UtApi {
        // Initialize the configuration for the Uploadthing service using the provided API key.
        let config = UploadthingConfig::builder().api_key(api_key).build();

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
            .header(header::USER_AGENT, self.config.user_agent.as_ref().unwrap()) // Set the User-Agent header.
            .header(
                "x-uploadthing-api-key",
                self.config.api_key.as_ref().unwrap(), // Set the custom API key header.
            )
            .header(
                "x-uploadthing-version",
                self.config.version.as_ref().unwrap(), // Set the custom version header.
            )
            .header("Cache-Control", "no-store") // Ensure the response is not cached.
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
}
