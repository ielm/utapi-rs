# utapi-rs

A high-level, ergonomic Rust crate for interacting with the Uploadthing API.

## Why?

If you're using Rust and want to use Uploadthing for file uploading, `utapi-rs` streamlines the process by providing a set of convenient functions mirroring Uploadthing's API.

## Setup

Add the following to your `Cargo.toml` file to include `utapi-rs` as a dependency:

```toml
[dependencies]
utapi-rs = "0.1.0"
```

## Usage

Below is a quick example of using `utapi-rs` to list files and delete a file.

```rust
use utapi_rs::{UtApi, ListFilesOpts, DeleteFileResponse};

#[tokio::main]
async fn main() {
    // Assume `new` attempts to retrieve the API key from an environment variable.
    let api = UtApi::new(None).expect("API key must be set");

    // List the files
    let opts = Some(ListFilesOpts {
        limit: Some(10), // Just as an example, limits the results to 10 files
        offset: None, // No offset for this example
    });

    match api.list_files(opts).await {
        Ok(file_response) => println!("Files: {:?}", file_response),
        Err(e) => eprintln!("Error listing files: {:?}", e),
    }

    // Delete a file
    match api.delete_files(vec!["file_key_to_delete".to_string()]).await {
        Ok(DeleteFileResponse { success }) => println!("Successfully deleted file: {}", success),
        Err(e) => eprintln!("Error deleting file: {:?}", e),
    }
}
```

Ensure that you have the `tokio` async runtime in your dependencies, as this library is designed to work asynchronously.

## API Key

For security purposes, it is recommended not to hardcode the API key in your codebase. Instead, you should set it up as an environment variable:

```
# .env
UPLOADTHING_SECRET=sk_*************************
```

Make sure to load the `.env` file or export the environment variable for your runtime accordingly.

## Contributing

Contributions are welcome! Please read our [contributing guidelines](CONTRIBUTING.md) for more details.

## License

`utapi-rs` is released under the MIT License. See the [LICENSE](LICENSE) file for details.
