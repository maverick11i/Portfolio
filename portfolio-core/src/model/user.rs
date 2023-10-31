use derive_new::new;
use serde::{Deserialize, Serialize};

/// Struct representing GitHub user information.
#[derive(new, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// User's login name.
    pub login: Option<String>,

    /// User's unique ID.
    pub id: Option<i64>,

    /// User node ID.
    pub node_id: Option<String>,

    /// URL of the user's avatar (profile picture).
    pub avatar_url: Option<String>,

    /// Gravatar ID of the user.
    pub gravatar_id: Option<String>,

    /// URL of the user's profile page.
    pub url: Option<String>,

    /// URL of the user's GitHub HTML page.
    pub html_url: Option<String>,

    /// URL of the user's followers on GitHub.
    pub followers_url: Option<String>,

    /// URL of the user's followed users on GitHub.
    pub following_url: Option<String>,

    /// URL of the user's gists on GitHub.
    pub gists_url: Option<String>,

    /// URL of the user's starred repositories on GitHub.
    pub starred_url: Option<String>,

    /// URL of the user's subscriptions on GitHub.
    pub subscriptions_url: Option<String>,

    /// URL of the user's organizations on GitHub.
    pub organizations_url: Option<String>,

    /// URL of the user's repositories on GitHub.
    pub repos_url: Option<String>,

    /// URL of the user's events on GitHub.
    pub events_url: Option<String>,

    /// URL of the user's received events on GitHub.
    pub received_events_url: Option<String>,

    /// Type of the user account (e.g., "User" or "Organization").
    pub type_: Option<String>,

    /// Indicates whether the user is a site administrator.
    pub site_admin: Option<bool>,

    /// User's full name.
    pub name: Option<String>,

    /// User's company or organization.
    pub company: Option<String>,

    /// User's blog or website URL.
    pub blog: Option<String>,

    /// User's location.
    pub location: Option<String>,

    /// User's email address.
    pub email: Option<String>,

    /// Indicates whether the user is available for hire.
    pub hireable: Option<bool>,

    /// User's bio or profile description.
    pub bio: Option<String>,

    /// User's Twitter username.
    pub twitter_username: Option<String>,

    /// Number of public repositories owned by the user.
    pub public_repos: Option<i64>,

    /// Number of public gists owned by the user.
    pub public_gists: Option<i64>,

    /// Number of followers of the user.
    pub followers: Option<i64>,

    /// Number of users followed by the user.
    pub following: Option<i64>,

    /// Date and time when the user account was created.
    pub created_at: Option<String>,

    /// Date and time when the user account was last updated.
    pub updated_at: Option<String>,
}
