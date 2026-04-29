use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

static ALPHABET: [char; 54] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J',
    'K', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
];

/// Registration invitation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invitation {
    /// Unique invite code (used as _id)
    #[serde(rename = "_id")]
    pub code: String,
    /// Email address this invitation was issued to
    pub email: String,
    /// ID of the privileged user who created this invitation
    pub created_by: String,
    /// Whether the invitation has been redeemed
    pub used: bool,
    /// ID of the user who redeemed this invitation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_by: Option<String>,
    /// When the invitation was created
    pub created_at: Timestamp,
    /// Client vertical (e.g. "author", "home_services")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical: Option<String>,
    /// Intake form metadata collected at invite time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Invitation {
    pub fn new(
        email: String,
        created_by: String,
        vertical: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Invitation {
            code: nanoid::nanoid!(8, &ALPHABET),
            email,
            created_by,
            used: false,
            used_by: None,
            created_at: Timestamp::now_utc(),
            vertical,
            metadata,
        }
    }
}
