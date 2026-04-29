use revolt_database::{Database, User};
use revolt_result::{create_error, Result};
use rocket::{serde::json::Json, State};
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct InvitationRecord {
    pub code: String,
    pub email: String,
    pub created_by: String,
    pub used: bool,
    pub used_by: Option<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// # List Invitations
///
/// Returns all invitations. Requires privileged access.
#[openapi(tag = "Admin")]
#[get("/invitations")]
pub async fn list_invitations(
    db: &State<Database>,
    user: User,
) -> Result<Json<Vec<InvitationRecord>>> {
    if !user.privileged {
        return Err(create_error!(NotPrivileged));
    }

    let records = db
        .fetch_all_invitations()
        .await?
        .into_iter()
        .map(|inv| InvitationRecord {
            code: inv.code,
            email: inv.email,
            created_by: inv.created_by,
            used: inv.used,
            used_by: inv.used_by,
            created_at: inv.created_at.to_string(),
            vertical: inv.vertical,
            metadata: inv.metadata,
        })
        .collect();

    Ok(Json(records))
}
