//! `utapi-rs` is a Rust library that provides a set of APIs for interacting with the UploadThing service.
//!
//! The library offers various functionalities such as file uploading, file management, and
//! retrieving file URLs, which are designed to be used server-side.

/// This module defines the configuration structures for `utapi-rs`.
/// It includes all necessary configurations required to initialize and run the service.
pub mod config;

/// This module contains the data models used throughout the `utapi-rs` application.
/// These models represent the core data structures that are manipulated and stored
/// by the service.
pub mod models;

/// The core API module providing the main functionality of the `utapi-rs` service.
/// This module includes all the API endpoints and related logic to perform
/// the intended operations.
pub mod utapi;

/// Re-export the `UtApi` struct at the root of the crate for easier access by consumers.
/// This allows users of the `utapi-rs` library to interact with the API without
/// needing to traverse the module hierarchy.
pub use utapi::UtApi;
