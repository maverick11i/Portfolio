use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::prelude::*;

/// Represents the state of data fetching.
pub enum FetchState<T> {
    /// Indicates that data fetching is in progress.
    Fetching,
    /// Indicates that data fetching is not in progress.
    NotFetching,
    /// Represents a successful data fetch with the retrieved value.
    Success(T),
    /// Represents a fetch error with details.
    Failed(FetchError),
}

/// Example usage of the `FetchState` enum:
///
/// ```rust
/// use my_module::FetchState;
///
/// fn main() {
///     let data: FetchState<String> = FetchState::Success("Data fetched successfully".to_string());
///
///     match data {
///         FetchState::Fetching => println!("Fetching data..."),
///         FetchState::NotFetching => println!("Data not being fetched."),
///         FetchState::Success(value) => println!("Data: {}", value),
///         FetchState::Failed(error) => eprintln!("Error: {}", error),
///     }
/// }
/// ```
/// Represents an error that occurs during data fetching.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}
