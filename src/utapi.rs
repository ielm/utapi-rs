use crate::config::{ApiKey, UploadthingConfig};
use crate::models::{
    Acl, ContentDisposition, DeleteFileResponse, FileKeysPayload, FileObj, FileUpload,
    ListFilesOpts, PresignedUrlOpts, PresignedUrlResponse, RenameFilesOpts, UploadFileOpts,
    UploadFileResponse, UploadFileResponseData, UploadthingFileResponse, UploadthingUrlsResponse,
    UploadthingUsageInfo,
};
use anyhow::anyhow;
use filesize::PathExt;
use rand::{thread_rng, Rng};
use reqwest::{header, multipart, Client, Response};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

use std::time::Duration;
use tokio::macros::support::Future;
use tokio::task::JoinHandle;

const MAX_RETRIES: u32 = 20;
const MAXIMUM_BACKOFF_MS: u64 = 64 * 1000;

/// The `UtApi` struct represents the client for interacting with the Uploadthing API.
///
/// It contains the configuration for the service and the HTTP client used to make requests.
#[derive(Clone)]
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
    ) -> Result<Response, anyhow::Error> {
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
                self.config.api_key.as_ref().unwrap().to_string(), // Set the custom API key header.
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
            let response_data =
                serde_json::to_string_pretty(&response.json::<serde_json::Value>().await?)?.clone();
            Err(anyhow!(response_data))
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

    /// Uploads files to the `Uploadthing` service.
    pub async fn upload_files(
        &self,
        files: Vec<FileObj>,
        opts: Option<UploadFileOpts>,
        wait_until_done: bool,
    ) -> Result<Vec<FileUpload>, Box<dyn Error>> {
        let mut metadata = HashMap::new();
        let mut content_disposition = "inline";
        let mut acl = "public-read";

        match opts {
            None => {}
            Some(o) => {
                metadata = o.metadata.unwrap_or(HashMap::new());
                content_disposition = match o.content_disposition {
                    None => "inline",
                    Some(cd) => match cd {
                        ContentDisposition::Attachment => "attachment",
                        ContentDisposition::Inline => "inline",
                    },
                };
                acl = match o.acl {
                    None => "public-read",
                    Some(acl) => match acl {
                        Acl::Private => "private",
                        Acl::PublicRead => "public-read",
                    },
                }
            }
        }

        let value = self
            .upload_files_internal(files, metadata, content_disposition, acl, wait_until_done)
            .await?;
        Ok(value)
    }

    /// Ping UploadThing to send a message saying a file is going to be uploaded, then upload it.
    async fn upload_files_internal(
        &self,
        files: Vec<FileObj>,
        metadata: HashMap<String, String>,
        content_disposition: &str,
        acl: &str,
        wait_until_done: bool,
    ) -> Result<Vec<FileUpload>, anyhow::Error> {
        let file_data = files
            .iter()
            .map(|f| {
                let mime_type = mime_guess::from_path(&f.path);
                let mime = mime_type.first_or_octet_stream().to_string();
                serde_json::json!({
                    "name": f.name,
                    "type": mime,
                    "size": f.path.size_on_disk().expect("Should be able to get file size"),
                })
            })
            .collect::<Vec<_>>();

        let json_data = &json!({
            "files": file_data,
            "metadata": metadata,
            "contentDisposition": content_disposition,
            "acl": acl
        });
        let response = self
            .request_uploadthing("/api/uploadFiles", json_data)
            .await;

        let response = match response {
            Err(e) => {
                eprintln!("[UT] Error uploading files: {}", e);
                eprintln!(
                    "[UT] Data sent in request:\n{}",
                    serde_json::to_string(&json_data).unwrap()
                );
                return Err(e);
            }
            Ok(r) => r,
        };

        let uf_response: UploadFileResponse = response.json().await?;
        let mut handles = vec![];
        for (i, file) in files.iter().enumerate() {
            let presigned = uf_response.data[i].clone();
            let data = file_data[i].clone();
            let path = file.path.clone();
            let client = self.clone();
            let task: JoinHandle<Result<FileUpload, anyhow::Error>> = tokio::task::spawn(
                async move {
                    // TODO: handle multi files vs. single url
                    //
                    // Actual JS implementation:
                    //
                    // if ("urls" in presigned) {
                    // 	await uploadMultipart(file, presigned, {
                    // 		...opts
                    // 	});
                    // } else {
                    // 	await uploadPresignedPost(file, presigned, {
                    // 		...opts
                    // 	});
                    // }
                    let mut f = std::fs::File::open(&path)?;
                    let file_name = data["name"].as_str().unwrap().to_string();

                    tokio::select! {
                        result = client.upload_presigned_post(file_name.clone(), &mut f, &presigned) => {
                            match result {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("[UT] Error uploading file {:?}: {}", path, e);
                                    return Err(e);
                                }
                            }
                        }
                        _ = tokio::signal::ctrl_c() => {
                            eprintln!("[UT] Upload cancelled for file {:?}", path);
                            return Err(anyhow::anyhow!("Upload cancelled"));
                        }
                    }
                    if wait_until_done {
                        // Poll for file data
                        let url =
                            format!("{}/api/pollUpload/{}", client.config.host, presigned.key);

                        tokio::select! {
                            result = retry_with_time_delays(|| client.poll_for_file_data(&url)) => {
                                result?;
                            }
                            _ = tokio::signal::ctrl_c() => {
                                eprintln!("[UT] Polling cancelled for file {:?}", path);
                                return Err(anyhow::anyhow!("Polling cancelled"));
                            }
                        }
                    }

                    Ok(FileUpload {
                        key: presigned.key.clone(),
                        url: presigned.file_url.clone(),
                        name: file_name,
                        size: data["size"].as_u64().unwrap(),
                    })
                },
            );

            handles.push(task);
        }

        let uploads = futures::future::try_join_all(handles).await?;
        let uploads: Vec<FileUpload> = uploads.into_iter().filter_map(Result::ok).collect();

        Ok(uploads)
    }

    /// Uploads a file using a POST request to the Uploadthing service.
    async fn upload_presigned_post(
        &self,
        file_name: String,
        file: &mut std::fs::File,
        presigned: &UploadFileResponseData,
    ) -> Result<(), anyhow::Error> {
        let mut form = multipart::Form::new();

        for (k, v) in presigned.fields.as_object().unwrap().iter() {
            let value = v.clone().to_owned().as_str().unwrap().to_owned();
            form = form.text(k.clone(), value);
        }

        let mut file_bytes = Vec::new();
        file.read_to_end(&mut file_bytes)?;
        let file_part = multipart::Part::bytes(file_bytes).file_name(file_name.clone());
        form = form.part("file", file_part);

        let res = self
            .client
            .post(&presigned.presigned_url)
            .header(
                "x-uploadthing-api-key",
                self.config.api_key.as_ref().unwrap().to_string(),
            )
            .multipart(form)
            .send()
            .await?;

        if !res.status().is_success() {
            let text = res.text().await?;
            eprintln!("Failed to upload file: {}", text);
        }

        Ok(())
    }

    /// Make a request to UploadThing to check if the file has finished uploading.
    async fn poll_for_file_data(&self, url: &str) -> Result<Option<()>, anyhow::Error> {
        let res = self
            .client
            .get(url)
            .header(
                "x-uploadthing-api-key",
                self.config.api_key.as_ref().unwrap().to_string(),
            )
            .send()
            .await;

        let res = match res {
            Ok(res) => res,
            Err(err) => {
                println!("[UT] Error polling for file data for {}: {}", url, err);
                return Err(anyhow!(err));
            }
        };

        let maybe_json: Result<serde_json::Value, _> =
            res.json().await.map_err(|err| err.to_string());

        match maybe_json {
            Ok(json) => {
                if json["status"] == "done" {
                    return Ok(Some(()));
                }
            }
            Err(err) => {
                println!("[UT] Error polling for file data for {}: {}", url, err);
            }
        }

        Ok(None)
    }
}

/// Retry a function with exponential timed back-off.
async fn retry_with_time_delays<F, T, Fut>(do_the_thing: F) -> Result<Option<T>, anyhow::Error>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<Option<T>, anyhow::Error>>,
{
    let mut tries = 0;
    let mut backoff_ms = 500;
    let mut backoff_fuzz_ms: i32;

    loop {
        if tries > MAX_RETRIES {
            return Ok(None);
        }

        let result = do_the_thing().await;
        if result.is_ok() {
            return result;
        }

        tries += 1;
        backoff_ms = std::cmp::min(MAXIMUM_BACKOFF_MS, backoff_ms * 2);
        backoff_fuzz_ms = thread_rng().gen_range(0..500);

        if tries > 3 {
            println!(
                "[UT] Call unsuccessful after {} tries. Retrying in {} seconds...",
                tries,
                backoff_ms / 1000
            );
        }

        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                return Err(anyhow::anyhow!("Operation cancelled"));
            }
            _ = tokio::time::sleep(Duration::from_millis(backoff_ms + backoff_fuzz_ms as u64)) => {}
        }
    }
}
