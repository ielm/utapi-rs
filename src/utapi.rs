use reqwest::{header, Client, Response};
use serde::Serialize;
use std::error::Error;

use crate::config::UploadthingConfig;

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
}
