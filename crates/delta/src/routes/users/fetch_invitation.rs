use revolt_database::{AbstractInvitations, Database, User};
use revolt_result::Result;
use rocket::{serde::json::Json, State};
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct InvitationResponse {
    pub vertical: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// # Fetch Own Invitation
///
/// Returns the invitation used to create this account, including vertical and intake metadata.
#[openapi(tag = "User Information")]
#[get("/@me/invitation")]
pub async fn fetch_invitation(
    user: User,
    db: &State<Database>,
) -> Result<Json<InvitationResponse>> {
    let inv = db.fetch_invitation_by_user(&user.id).await?;
    Ok(Json(InvitationResponse {
        vertical: inv.vertical,
        metadata: inv.metadata,
    }))
}
