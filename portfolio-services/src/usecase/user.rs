use crate::api::requests::request_get;
use portfolio_core::model::status::FetchError;

/// Fetches the user profile information for a given username.
///
/// # Arguments
///
/// * `username` - The GitHub username of the user whose profile information is to be fetched.
///
/// # Returns
///
/// A `Result` containing the user profile information as a JSON string or a `FetchError` in case of failure.
pub async fn get_user_profile(username: &str) -> Result<String, FetchError> {
    request_get(&format!("/users/{}", username)).await
}
