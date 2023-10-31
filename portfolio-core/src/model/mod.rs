pub mod status;
pub mod user;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use user::User;

/// Represents error information for the Unprocessable Entity error in the Conduit API.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    /// A map of error field names to error messages.
    pub errors: HashMap<String, Vec<String>>,
}

/// Represents a wrapper for the DELETE operation in the Conduit API.
pub type DeleteWrapper = HashMap<(), ()>;
