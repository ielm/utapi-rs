use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Configuration for the Uploadthing service.
///
/// This struct contains all the necessary configurations required
/// to interact with the Uploadthing API. It includes the host URL,
/// optional user agent, API key, and version information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadthingConfig {
    /// The host URL of the Uploadthing service.
    pub host: String,
    /// An optional user agent string to be sent with each request.
    /// This can be used for identifying the client to the server.
    pub user_agent: Option<String>,
    /// An optional API key for authentication with the Uploadthing service.
    /// If provided, it will be included in the headers of each request.
    pub api_key: Option<ApiKey>,
    /// An optional version string to be sent with each request.
    /// This can represent the version of the client application.
    pub version: Option<String>,
}

/// The version of the current crate, taken directly from the Cargo package metadata.
/// This constant is used to provide version information in user agents and other
/// parts of the application that may require knowledge of the current version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize)]
/// Represents an API key used for authenticating with the Uploadthing service.
///
/// This struct holds the actual API key and an optional prefix.
/// The prefix can be used to add a specific identifier before the key
/// when sending it in the request header, but it is not required.
#[derive(Clone)]
pub struct ApiKey {
    /// An optional prefix to be added to the API key.
    /// This can be used to distinguish between different types of keys.
    pub prefix: Option<String>,

    /// The API key string that is used for authentication.
    pub key: String,
}

impl ApiKey {
    /// Constructs an `ApiKey` instance from an environment variable.
    ///
    /// This function attempts to create an `ApiKey` by reading the value
    /// of the environment variable `UPLOADTHING_SECRET`. If the variable
    /// is set, it returns `Some(ApiKey)` with the key set to the value
    /// of the environment variable, and no prefix. If the environment
    /// variable is not set, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming the environment variable `UPLOADTHING_SECRET` is set to "secret123"
    /// let api_key = ApiKey::from_env().unwrap();
    /// assert_eq!(api_key.key, "secret123");
    /// assert_eq!(api_key.prefix, None);
    /// ```
    pub fn from_env() -> Option<ApiKey> {
        match std::env::var("UPLOADTHING_SECRET") {
            Ok(key) => Some(ApiKey { prefix: None, key }),
            Err(_) => None,
        }
    }
}

impl Default for ApiKey {
    /// Provides a default `ApiKey` by trying to read from the environment variable.
    ///
    /// This implementation uses `ApiKey::from_env()` to attempt to create an `ApiKey`.
    /// If the environment variable is not set, this will panic. It is generally expected
    /// that the environment variable is set if this method is used.
    ///
    /// # Panics
    ///
    /// This function will panic if the environment variable `UPLOADTHING_SECRET` is not set.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming the environment variable `UPLOADTHING_SECRET` is set
    /// let api_key = ApiKey::default();
    /// // Use the `api_key` as needed
    /// ```
    fn default() -> ApiKey {
        Self::from_env().expect("UPLOADTHING_SECRET environment variable is not set")
    }
}

impl FromStr for ApiKey {
    type Err = ();

    /// Creates an `ApiKey` instance from a string slice.
    ///
    /// This function attempts to parse the given string slice into an `ApiKey`
    /// by checking for a colon which separates the optional prefix and the key.
    /// If a colon is found, the part before the colon is set as the prefix and
    /// the part after the colon is set as the key. If no colon is found, the
    /// entire string is set as the key with no prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// let api_key: ApiKey = "Bearer:secret123".parse().unwrap();
    /// assert_eq!(api_key.prefix, Some("Bearer".to_string()));
    /// assert_eq!(api_key.key, "secret123".to_string());
    ///
    /// let api_key: ApiKey = "secret123".parse().unwrap();
    /// assert_eq!(api_key.prefix, None);
    /// assert_eq!(api_key.key, "secret123".to_string());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        let (prefix, key) = match parts.as_slice() {
            [prefix, key] => (Some(prefix.to_string()), key.to_string()),
            [key] => (None, key.to_string()),
            _ => return Err(()),
        };

        Ok(ApiKey { prefix, key })
    }
}

impl std::fmt::Display for ApiKey {
    /// Formats the `ApiKey` for display purposes.
    ///
    /// This implementation will only display the `key` field of the `ApiKey`.
    /// The prefix is not included in the output. This is generally used when
    /// the API key needs to be included in a header or similar context where
    /// the prefix is not required.
    ///
    /// # Examples
    ///
    /// ```
    /// let api_key = ApiKey {
    ///     prefix: Some(String::from("Bearer")),
    ///     key: String::from("secret123"),
    /// };
    /// assert_eq!(format!("{}", api_key), "secret123");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

impl UploadthingConfig {
    /// Creates a new `UploadthingConfig` with default values.
    ///
    /// This is a convenience method that calls `UploadthingConfig::default()`
    /// to create a new configuration instance with the default host, user agent,
    /// and version values. The API key is not set by default.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = UploadthingConfig::new();
    /// // The `config` now contains the default settings.
    /// ```
    pub fn new() -> UploadthingConfig {
        UploadthingConfig::default()
    }

    /// Creates a builder for `UploadthingConfig`.
    ///
    /// This method returns an instance of `UploadthingConfigBuilder` which can be
    /// used to set various configuration options before building the final
    /// `UploadthingConfig` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = UploadthingConfig::builder();
    /// let config = builder.host("https://customhost.com")
    ///                     .user_agent("CustomUserAgent/1.0")
    ///                     .api_key("my_api_key")
    ///                     .version("1.0.0")
    ///                     .build();
    /// // The `config` is now customized with the provided settings.
    /// ```
    pub fn builder() -> UploadthingConfigBuilder {
        UploadthingConfigBuilder::new()
    }
}

impl Default for UploadthingConfig {
    /// Provides default values for `UploadthingConfig`.
    ///
    /// This implementation sets default values for the host, user agent, and version.
    /// The default host is set to "<https://uploadthing.com>". The user agent is constructed
    /// using the crate version and the name "utapi-rs". The version is set to the crate's
    /// current version. The API key is attempted to be retrieved from the environment
    /// variable `UPLOADTHING_SECRET`. If the environment variable is not set, the API key
    /// will be `None`.
    ///
    /// # Returns
    ///
    /// Returns an `UploadthingConfig` instance with default values set.
    ///
    /// # Examples
    ///
    /// ```
    /// let default_config = UploadthingConfig::default();
    /// assert_eq!(default_config.host, "https://uploadthing.com");
    /// // Other fields are set to their respective defaults
    /// ```
    fn default() -> UploadthingConfig {
        UploadthingConfig {
            host: "https://uploadthing.com".to_string(),
            // User agent includes the crate name and version for identification purposes.
            user_agent: Some(format!("utapi-rs/{}/rust", VERSION)),
            // Attempt to retrieve the API key from the environment variable.
            // api_key: std::env::var("UPLOADTHING_SECRET").ok(),
            api_key: ApiKey::from_env(),
            // Version is set to the current crate version.
            version: Some(VERSION.to_string()),
        }
    }
}

/// A builder for `UploadthingConfig`.
///
/// This builder allows for a fluent interface to construct a `UploadthingConfig`
/// instance with custom values. It starts with default values and allows the
/// user to set properties such as host, user agent, API key, and version
/// before building the final configuration object.
pub struct UploadthingConfigBuilder {
    // Internal `UploadthingConfig` to apply settings to.
    config: UploadthingConfig,
}

impl UploadthingConfigBuilder {
    /// Creates a new `UploadthingConfigBuilder` with default configuration values.
    ///
    /// This uses `UploadthingConfig::default()` to initialize the internal config
    /// with default values.
    pub fn new() -> Self {
        UploadthingConfigBuilder {
            config: UploadthingConfig::default(),
        }
    }

    /// Sets the host URL in the configuration.
    ///
    /// # Arguments
    ///
    /// * `host` - A string slice that holds the URL of the Uploadthing service.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = UploadthingConfigBuilder::new().host("https://example.com");
    /// ```
    pub fn host(mut self, host: &str) -> Self {
        self.config.host = host.to_string();
        self
    }

    /// Sets the user agent in the configuration.
    ///
    /// # Arguments
    ///
    /// * `user_agent` - A string slice that represents the user agent to be sent with each request.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = UploadthingConfigBuilder::new().user_agent("MyUploader/1.0");
    /// ```
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.config.user_agent = Some(user_agent.to_string());
        self
    }

    /// Sets the API key in the configuration.
    ///
    /// This method allows the user to provide an API key as a string slice
    /// that will be used for authenticating with the Uploadthing service.
    /// It attempts to parse the provided string into an `ApiKey` instance
    /// using the `FromStr` trait implementation for `ApiKey`. If parsing is
    /// successful, the `ApiKey` is wrapped in a `Some` and set in the configuration.
    /// If parsing fails, it falls back to the default `ApiKey` which is obtained
    /// from the environment variable `UPLOADTHING_SECRET`.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice that represents the API key for authenticating with the service.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = UploadthingConfigBuilder::new().api_key("your_api_key");
    /// ```
    ///
    /// # Panics
    ///
    /// This method will panic if the provided `api_key` string is not in a valid format
    /// and there is no `UPLOADTHING_SECRET` environment variable set to fall back on.
    pub fn api_key(mut self, api_key: &str) -> Self {
        // Attempt to parse the provided API key string.
        // If parsing fails, fall back to the default which may panic if the environment variable is not set.
        self.config.api_key = Some(ApiKey::from_str(api_key).unwrap_or_default());
        self
    }

    /// Sets the version in the configuration.
    ///
    /// # Arguments
    ///
    /// * `version` - A string slice that represents the version of the client application.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = UploadthingConfigBuilder::new().version("2.0.0");
    /// ```
    pub fn version(mut self, version: &str) -> Self {
        self.config.version = Some(version.to_string());
        self
    }

    /// Builds the `UploadthingConfig` with the current settings of the builder.
    ///
    /// Consumes the builder and returns the configured `UploadthingConfig` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = UploadthingConfigBuilder::new()
    ///     .host("https://example.com")
    ///     .user_agent("MyUploader/1.0")
    ///     .api_key("your_api_key")
    ///     .version("2.0.0")
    ///     .build();
    /// ```
    pub fn build(self) -> UploadthingConfig {
        self.config
    }
}

/// Implements the `Default` trait for `UploadthingConfigBuilder`.
///
/// This allows for the creation of a builder with the default configuration
/// by calling `UploadthingConfigBuilder::default()`.
impl Default for UploadthingConfigBuilder {
    /// Returns a new `UploadthingConfigBuilder` initialized with default values.
    fn default() -> Self {
        Self::new()
    }
}
