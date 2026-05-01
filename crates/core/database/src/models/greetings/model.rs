use serde::{Deserialize, Serialize};

/// A vertical-specific onboarding greeting sent to #start-here by Otto
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Greeting {
    /// Vertical key used as _id (e.g. "author", "home_services", "default")
    #[serde(rename = "_id")]
    pub vertical: String,
    /// Message template — use {username} for interpolation
    pub message: String,
}
