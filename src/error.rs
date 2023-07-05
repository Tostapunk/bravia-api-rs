//! A composite error type for errors that can occur while interacting with the server.

use derive_builder::UninitializedFieldError;
use serde::Deserialize;
use std::fmt;

/// Alias to a Result containing a local Error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents a specific error returned from a server API call.
#[derive(Debug, Deserialize)]
pub struct BraviaErrorCode {
    /// The numeric error code returned by the server.\
    /// For details on the error codes, please see the
    /// [official documentation](https://pro-bravia.sony.net/develop/integrate/rest-api/spec/errorcode-list/index.html).
    pub code: usize,
    /// The error message returned by the server.
    pub message: String,
}

impl fmt::Display for BraviaErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}: {}", self.code, self.message)
    }
}

/// A set of errors that can occur when interacting with the server.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// One or more required fields without a value. The enclosed error was returned from `derive_builder`.
    #[error("An error occurred during initialization")]
    FieldInitialization(#[from] UninitializedFieldError),
    /// The web request experienced an error. The enclosed error was returned from `reqwest`.
    #[error("NetworkError: {}", _0)]
    NetworkError(#[from] reqwest::Error),
    /// The response from the server gave a response code that indicated an error.
    #[error("Error status received: {}", _0)]
    BadStatus(reqwest::StatusCode),
    /// An expected value was missing from the response.
    #[error("Value missing from response: {}", _0)]
    MissingValue(&'static str),
    /// Invalid response format (`result` and `error` fields was missing).
    #[error("Invalid response received: {}", _0)]
    InvalidResponse(&'static str),
    /// An error occurred while loading the JSON response. The enclosed error was returned from `serde_json`
    #[error("JSON deserialize error: {}", _0)]
    DeserializeError(#[from] serde_json::Error),
    /// Errors returned by the server.
    #[error("Error returned by Bravia: {}", _0)]
    BraviaError(BraviaErrorCode),
    /// The requested API service was not found.
    #[error("API service not found.")]
    BraviaApiServiceNotFound,
    /// The requested API was not found in the service.
    #[error("API not found.")]
    BraviaApiNotFound,
    /// The requested API version was not supported by the server.
    #[error("This API version is not supported.")]
    BraviaApiLevelError,
    /// Wrong or absent password for the requested authentication level.
    #[error("A password is required in order to access this API")]
    BraviaAuthLevelError,
}
